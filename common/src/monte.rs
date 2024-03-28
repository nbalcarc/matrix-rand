use rand::{seq::SliceRandom, thread_rng};
use crate::regular;


/// Downscale through random sampling
pub fn multiply(a: &[f32], b: &[f32], sizes: (usize, usize, usize), downsample: usize) -> Vec<f32> {
    // note: sizes = (A height, A width && B height, B width)

    // turn A into columns for ease of access
    let mut a_columns: Vec<Vec<f32>> = (0..sizes.1).into_iter() //columns of A
        .map(|_| Vec::with_capacity(sizes.0))
        .collect();
    for i in 0..a.len() {
        a_columns[i % sizes.1].push(a[i]);
    }

    // get column magnitudes
    let column_squared_magnitudes: Vec<f64> = a_columns.iter()
        .map(|col|
             col.iter()
                 .map(|x| (*x as f64) * (*x as f64))
                 .sum()
        )
        .collect();

    // generate probabilities for each column
    let bracket_a: f64 = column_squared_magnitudes.iter().sum();
    let probs: Vec<f64> = column_squared_magnitudes.iter().map(|x| x / bracket_a).collect();

    // select the columns to use
    let mut rng = thread_rng();
    let columns: Vec<usize> = (0..sizes.1).collect();
    let zipped: Vec<((usize, f64), f64)> = columns.iter()
        .zip(probs.iter())
        .map(|x| ((*x.0, *x.1), *x.1))
        .collect();
    let chosen: Vec<(usize, f64)> = (0..downsample).into_iter()
        .map(|_| zipped.choose_weighted(&mut rng, |x| x.1).unwrap().0)
        .collect();

    // fix columns and push to S and R
    let mut s = Vec::with_capacity(sizes.0 * downsample); //new A
    let mut r = Vec::with_capacity(sizes.2 * downsample); //new B
    for choice in chosen {
        let index = choice.0;
        let prob = choice.1;

        let adjusted_scol: Vec<f32> = a_columns[index].iter()
            .map(|x| x / (downsample as f32 * prob as f32).sqrt())
            .collect();
        let adjusted_rcol: Vec<f32> = b[sizes.2 * index..sizes.2 * (index+1)].iter()
            .map(|x| x / (downsample as f32 * prob as f32).sqrt())
            .collect();
        s.extend(adjusted_scol);
        r.extend(adjusted_rcol);
    }

    // return final result
    regular::multiply_float(&s, &r, (sizes.0, downsample, sizes.2))
}



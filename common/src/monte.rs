use rand::{seq::SliceRandom, thread_rng};

use crate::regular;



/// Downscale through random sampling
pub fn multiply_float(a: &[f32], b: &[f32], sizes: (usize, usize, usize), downsample: usize) -> Vec<f32> {
    // note: sizes = (A height, A width && B height, B width)
    //let mut c = vec![0.0; sizes.0 * sizes.2];
    //let (m, n, p) = sizes;

    //let columns = Vec::with_capacity(sizes.1);
    let mut a_columns: Vec<Vec<f32>> = (0..sizes.1).into_iter() //columns of A
        .map(|_| Vec::with_capacity(sizes.0))
        .collect();
    for i in 0..a.len() {
        a_columns[i % sizes.1].push(a[i]);
    }


    //todo!();

    // TODO: CONSIDER USING AN ITERATOR WITH CHUNKING HERE INSTEAD OF A LOOP
    //println!("{}, {}, {}", a.len(), sizes.1, a.len() / sizes.0);
    let mut column_squared_magnitudes = vec![0.0; sizes.1]; //used to calculate magnitude of each column
    for i in 0..a.len() { //iterate through all values in A
        column_squared_magnitudes[i / sizes.0] += a[i] as f64 * a[i] as f64;
    }

    let bracket_a: f64 = column_squared_magnitudes.iter().sum();
    let probs: Vec<f64> = column_squared_magnitudes.iter().map(|x| x / bracket_a).collect();
    //println!("{:?}, {:?}", probs, probs.iter().sum::<f64>());

    let columns: Vec<usize> = (0..sizes.1).collect();

    //columns.choose_multiple_weighted(&mut thread_rng(), amount, weight);
    let mut rng = thread_rng();
    //zip(columns, probs);
    let zipped: Vec<((usize, f64), f64)> = columns.iter()
        .zip(probs.iter())
        .map(|x| ((*x.0, *x.1), *x.1))
        .collect();
    let chosen: Vec<(usize, f64)> = (0..downsample).into_iter()
        .map(|_| zipped.choose_weighted(&mut rng, |x| x.1).unwrap().0)
        .collect();

    //println!("{:?}", chosen);

    //let mut s = vec![0.0; sizes.0 * downsample]; //new A
    //let mut r = vec![0.0; sizes.2 * downsample]; //new B
    let mut s = Vec::with_capacity(sizes.0 * downsample); //new A
    let mut r = Vec::with_capacity(sizes.2 * downsample); //new B

    for choice in chosen {
        let index = choice.0;
        let prob = choice.1;
        //s.extend_from_slice(&a[(choice..).step_by(sizes.1)]);
        //let indices: Vec<usize> = (choice..a.len()).step_by(sizes.1).collect();
        //for index in indices {
        //    s.push(a[index]);
        //}
        //r.extend_from_slice(&a[(choice..).step_by(sizes.1)]);

        //s.extend_from_slice(&a_columns[choice]);
        //r.extend_from_slice(&b[sizes.2 * choice..sizes.2 * (choice+1)]);
        let adjusted_scol: Vec<f32> = a_columns[index].iter()
            .map(|x| x / (downsample as f32 * prob as f32).sqrt())
            .collect();
        let adjusted_rcol: Vec<f32> = b[sizes.2 * index..sizes.2 * (index+1)].iter()
            .map(|x| x / (downsample as f32 * prob as f32).sqrt())
            .collect();
        s.extend(adjusted_scol);
        r.extend(adjusted_rcol);
    }

    let c = regular::multiply_float(&s, &r, (sizes.0, downsample, sizes.2));

    c

    //todo!();
    

    //regular::multiply_float(a, b, sizes)



    //for i in 0..m { //A height
    //    for j in 0..p { //B width
    //        // note: chooses a row in A and a column in B and sums the products
    //        let mut sum = 0.0;
    //        for k in 0..n { //A width and B height
    //            sum += a[i*n + k] * b[k*p + j];
    //        }
    //        c[i*p + j] = sum;
    //    }
    //}

    //c
}



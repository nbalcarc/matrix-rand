use rand::thread_rng;



/// Downscale through random sampling
pub fn multiply_float(a: &[f32], b: &[f32], sizes: (usize, usize, usize), s_size: u32) -> Vec<f32> {
    // note: sizes = (A height, A width && B height, B width)
    let mut c = vec![0.0; sizes.0 * sizes.2];
    let (m, n, p) = sizes;
    //let bracket_a: f64 = a.iter().map(|n| (n * n) as f64).sum();

    // TODO: CONSIDER USING AN ITERATOR WITH CHUNKING HERE INSTEAD OF A LOOP
    println!("{}, {}, {}", a.len(), sizes.1, a.len() / sizes.0);
    let mut column_squared_magnitudes = vec![0.0; sizes.1]; //used to calculate magnitude of each column
    for i in 0..a.len() { //iterate through all values in A
        //let squared = a[i] as f64 * a[i] as f64;
        //bracket_sum += squared;
        column_squared_magnitudes[i / sizes.0] += a[i] as f64 * a[i] as f64;
    }

    let bracket_a: f64 = column_squared_magnitudes.iter().sum();
    //let column_magnitudes: Vec<f64> = column_squares_sum.iter().map(|x| x.sqrt()).collect();

    //let mut probs = vec![0.0; sizes.1]; //weight of each column
    let probs: Vec<f64> = column_squared_magnitudes.iter().map(|x| x / bracket_a).collect();
    println!("{:?}, {:?}", probs, probs.iter().sum::<f64>());

    let columns: Vec<usize> = (0..sizes.1).collect();



    for i in 0..m { //A height
        for j in 0..p { //B width
            // note: chooses a row in A and a column in B and sums the products
            let mut sum = 0.0;
            for k in 0..n { //A width and B height
                sum += a[i*n + k] * b[k*p + j];
            }
            c[i*p + j] = sum;
        }
    }

    c
}



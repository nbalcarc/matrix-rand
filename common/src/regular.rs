use crate::neuralnet::MatrixMultiplier;

pub struct RegularMultiplier {}
impl MatrixMultiplier for RegularMultiplier {
    fn multiply(&self, a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
        multiply_float(a, b, sizes)
    }
}


/// Standard matrix multiplication
pub fn multiply_float(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
    // note: sizes = (A height, A width && B height, B width)
    let mut c = vec![0.0; sizes.0 * sizes.2];
    let (m, n, p) = sizes;

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


/// Standard matrix multiplication
pub fn multiply_int(a: &[i32], b: &[i32], sizes: (usize, usize, usize)) -> Vec<i32> {
    // note: sizes = (A height, A width && B height, B width)
    let mut c = vec![0; sizes.0 * sizes.2];
    let (m, n, p) = sizes;

    for i in 0..m { //A height
        for j in 0..p { //B width
            // note: chooses a row in A and a column in B and sums the products
            let mut sum = 0;
            for k in 0..n { //A width and B height
                sum += a[i*n + k] * b[k*p + j];
            }
            c[i*p + j] = sum;
        }
    }

    c
}



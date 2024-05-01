use rand::prelude::*;

use crate::neuralnet::Multiplier;

pub struct CountSketch {
    pub sketch: Vec<Vec<f32>>,
    pub hash_functions: Vec<(usize, usize)>, // (a, b) parameters for hash functions
}
impl Multiplier for CountSketch {
    fn multiply(&mut self, a: &[f32], b: &[f32], sizes: (usize, usize, usize), c: &mut [f32]) {
        self.countsketch_multiply(a, b, sizes, c);
    }
}
impl CountSketch {
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        let mut rng = thread_rng();
        let sketch = vec![vec![0.0; num_cols]; num_rows];

        // Generate hash functions (a, b) for each row
        let hash_functions: Vec<(usize, usize)> = (0..num_rows)
            .map(|_| (rng.gen_range(1..=usize::MAX), rng.gen_range(0..=usize::MAX)))
            .collect();

        CountSketch {
            sketch,
            hash_functions,
        }
    }

    pub fn sketch_vector(&self, column: &[f32]) -> Vec<f32> {
        // Ensure column length matches sketch rows
        assert_eq!(column.len(), self.sketch.len());

        let mut sketch_result = vec![0.0; self.sketch[0].len()];

        // Update CountSketch with the given column vector
        for i in 0..self.sketch.len() {
            let (a, b) = self.hash_functions[i];
            let hash_index = ((a as usize * i) + b as usize) % self.sketch[i].len();

            // Update sketch result at the computed hash index
            let sign = if a % 2 == 0 { 1 } else { -1 };
            sketch_result[hash_index] += column[i] * sign as f32;
        }

        sketch_result
    }

    pub fn countsketch_multiply(&self, a: &[f32], b: &[f32], sizes: (usize, usize, usize), c: &mut [f32]) {
        //let mut result = vec![0.0; sizes.0 * sizes.2];

        for i in 0..sizes.1 {
            // Sketch the i-th column of matrix a
            let sketch_a = self.sketch_vector(&a[i * sizes.0..(i + 1) * sizes.0]);

            for j in 0..sizes.2 {
                // Compute dot product of sketched column and j-th column of matrix b
                let mut sum = 0.0;
                for k in 0..sizes.0 {
                    sum += sketch_a[k] * b[k * sizes.2 + j];
                }
                c[i * sizes.2 + j] += sum;
            }
        }

    }
}

//pub fn matrix_multiply_with_sketch(
//    a: &[f32],
//    b: &[f32],
//    rows_a: usize,
//    cols_a: usize,
//    cols_b: usize,
//    count_sketch: &CountSketch,
//) -> Vec<f32> {
//    assert_eq!(a.len(), rows_a * cols_a);
//    assert_eq!(b.len(), cols_a * cols_b);
//
//    let mut result = vec![0.0; rows_a * cols_b];
//
//    for i in 0..cols_a {
//        // Sketch the i-th column of matrix a
//        let sketch_a = count_sketch.sketch_vector(&a[i * rows_a..(i + 1) * rows_a]);
//
//        for j in 0..cols_b {
//            // Compute dot product of sketched column and j-th column of matrix b
//            let mut sum = 0.0;
//            for k in 0..rows_a {
//                sum += sketch_a[k] * b[k * cols_b + j];
//            }
//            result[i * cols_b + j] += sum;
//        }
//    }
//
//    result
//}

//fn main() {
//    let num_rows = 5;
//    let num_cols = 5;
//    let count_sketch = CountSketch::new(num_rows, num_cols);
//
//    // Example input matrices dimensions
//    let rows_a = 3;
//    let cols_a = 2;
//    let cols_b = 4;
//
//    // Example input matrices
//    let matrix_a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]; // Row-major order
//    let matrix_b = vec![9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0]; // Row-major order
//
//    // Perform matrix multiplication with CountSketch
//    let product = matrix_multiply_with_sketch(
//        &matrix_a,
//        &matrix_b,
//        rows_a,
//        cols_a,
//        cols_b,
//        &count_sketch,
//    );
//
//    // Display the resulting matrix product
//    println!("Matrix Multiplication Result:");
//    for i in 0..rows_a {
//        for j in 0..cols_b {
//            print!("{:.2} ", product[i * cols_b + j]);
//        }
//        println!();
//    }
//}



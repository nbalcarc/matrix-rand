use rand::prelude::*;

// Define a struct for the CountSketch data structure
pub struct CountSketch {
    sketch: Vec<Vec<f32>>,
    hash_indices: Vec<Vec<usize>>,
    signs: Vec<Vec<i32>>,
}

impl CountSketch {
    // Constructor to initialize CountSketch with specified parameters
    pub fn new(rows: usize, sketch_width: usize, num_hash_functions: usize) -> Self {
        let mut sketch = vec![vec![0.0; sketch_width]; rows];
        let mut rng = thread_rng();
        let hash_indices: Vec<Vec<usize>> = (0..num_hash_functions)
            .map(|_| (0..rows).map(|_| rng.gen_range(0..sketch_width)).collect())
            .collect();
        let signs: Vec<Vec<i32>> = (0..num_hash_functions)
            .map(|_| (0..rows).map(|_| if rng.gen::<bool>() { 1 } else { -1 }).collect())
            .collect();

        CountSketch {
            sketch,
            hash_indices,
            signs,
        }
    }

    // Update the CountSketch with a column vector and corresponding sign using hash indices
    fn update_with_hash_indices(&mut self, column: &[f32], sign: i32, hash_indices: &[usize]) {
        assert_eq!(column.len(), self.sketch[0].len());
        for (i, &index) in hash_indices.iter().enumerate() {
            self.sketch[i][index] += column[i] * sign as f32;
        }
    }

    // Perform approximate matrix multiplication using CountSketch
    pub fn matrix_multiply(
        &mut self,
        A: &[f32],      // Contiguous array representing matrix A (row-major order)
        B: &[f32],      // Contiguous array representing matrix B (row-major order)
        sizes: (usize, usize, usize)
    ) -> Vec<f32> {
        let rows_A = sizes.0;
        let cols_A = sizes.1;
        let cols_B = sizes.2;
        assert_eq!(cols_A, B.len() / cols_B, "Matrix dimensions mismatch");

        let mut result = vec![0.0; rows_A * cols_B];

        // remove any borrow errors
        let hash_indices_copy = self.hash_indices.clone();
        let signs_copy = self.signs.clone();

        // Iterate over columns of B (result matrix columns)
        for j in 0..cols_B {
            // Compute hash indices and signs for the j-th column of B
            //let hash_indices = &self.hash_indices[j % self.hash_indices.len()];
            let hash_indices = &hash_indices_copy[j % self.hash_indices.len()];
            let signs = &signs_copy[j % self.signs.len()];

            // Update CountSketch with the j-th column of A and corresponding signs
            for i in 0..rows_A {
                let a_start_idx = i * cols_A;
                let b_start_idx = j * cols_A; // Start of j-th column in B
                let a_row = &A[a_start_idx..a_start_idx + cols_A];
                let b_col = &B[b_start_idx..b_start_idx + cols_A]; // Extract j-th column from B

                self.update_with_hash_indices(a_row, signs[i], hash_indices);

                // Compute the approximate product (accumulate into result)
                let result_idx = i * cols_B + j;
                for (k, &sketch_index) in hash_indices.iter().enumerate() {
                    result[result_idx] += self.sketch[k][sketch_index] * b_col[k];
                }

                self.update_with_hash_indices(a_row, -signs[i], hash_indices); // Restore the sketch after use
            }
        }

        result
    }
}

fn main() {
    // Example matrices A and B (contiguous arrays with row-major order)
    let A = vec![
        1.0, 2.0,
        3.0, 4.0,
    ];

    let B = vec![
        5.0, 6.0,
        7.0, 8.0,
    ];

    let rows_A = 2;   // Number of rows in matrix A
    let cols_A = 2;   // Number of columns in matrix A
    let cols_B = 2;   // Number of columns in matrix B

    // Parameters for CountSketch
    let sketch_width = 5;             // Width of the CountSketch
    let num_hash_functions = 3;        // Number of hash functions to use

    // Initialize CountSketch
    let mut count_sketch = CountSketch::new(rows_A, sketch_width, num_hash_functions);

    let sizes = (rows_A, cols_A, cols_B);

    // Perform approximate matrix multiplication using CountSketch
    let result = count_sketch.matrix_multiply(&A, &B, sizes);

    // Output the approximate result vector
    println!("Approximate result of matrix multiplication using CountSketch:");
    for i in 0..rows_A {
        for j in 0..cols_B {
            print!("{:.2} ", result[i * cols_B + j]);
        }
        println!();
    }
}



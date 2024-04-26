use rand::prelude::*;

// Function to perform Randomized Hadamard Transform (RHT) for matrix multiplication approximation
fn randomized_hadamard_transform(
    a: &[f64], // Contiguous array representing matrix A (row-major order)
    b: &[f64], // Contiguous array representing matrix B (row-major order)
    //m: usize,  // Number of rows in matrix A
    //n: usize,  // Number of columns in matrix A (and rows in matrix B)
    //p: usize,  // Number of columns in matrix B
    sizes: (usize, usize, usize), //rows in A, columns in A / rows in B, columns in B
    d: usize,  // Number of columns to sample for RHT approximation
) -> Vec<f64> {

    let m = sizes.0;
    let n = sizes.1;
    let p = sizes.2;

    // Generate random indices to sample d columns from matrices A and B
    let mut rng = thread_rng();
    let a_indices: Vec<usize> = (0..n).choose_multiple(&mut rng, d);
    let b_indices: Vec<usize> = (0..n).choose_multiple(&mut rng, d);

    // Initialize result vector
    let mut result = vec![0.0; m * p];

    // Perform Randomized Hadamard Transform approximation
    for idx in 0..d {
        let a_col_start = a_indices[idx] * m;
        let b_col_start = b_indices[idx] * p;

        for i in 0..m {
            for j in 0..p {
                result[i * p + j] += a[a_col_start + i] * b[b_col_start + j];
            }
        }
    }

    result
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

    //let m = 2; // Number of rows in matrix A
    //let n = 2; // Number of columns in matrix A (and rows in matrix B)
    //let p = 2; // Number of columns in matrix B
    let sizes = (2, 2, 2);
    let d = 2; // Number of columns to sample for RHT approximation

    // Perform Randomized Hadamard Transform approximation for matrix multiplication
    let approx_result = randomized_hadamard_transform(&A, &B, sizes, d);

    // Output the approximate result vector
    println!("Approximate result of matrix multiplication using RHT:");
    for i in 0..sizes.0 {
        for j in 0..sizes.2 {
            print!("{:.2} ", approx_result[i * sizes.2 + j]);
        }
        println!();
    }
}



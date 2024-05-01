mod csv_reader;
mod simple;
mod complex;

use crate::{complex::run_complex_disease, simple::run_simple};


fn short() {
    let a = [
        3.0, 2.0, 1.0, 5.0,
        9.0, 1.0, 3.0, 0.0,
    ];
    let b = [
        2.0, 9.0, 0.0,
        1.0, 3.0, 5.0,
        2.0, 4.0, 7.0,
        8.0, 1.0, 5.0,
    ];
    let sizes = (2, 4, 3);
    //println!("{:?}", regular::multiply_float(&a, &b, sizes));
    //println!("{:?}", monte::multiply(&a, &b, sizes, 3));
}


fn main() {
    println!("Hello, world!");
    //run_simple();
    run_complex_disease();
}



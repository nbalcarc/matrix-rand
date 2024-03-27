use common::{monte, regular};
use rand;


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
    println!("{:?}", regular::multiply_float(&a, &b, sizes));
    println!("{:?}", monte::multiply_float(&a, &b, sizes, 3));
}


fn long() {
    let sizes = (2_048, 1_536, 1_024);
    let a: Vec<f32> = (0..sizes.0*sizes.1).into_iter().map(|_| rand::random()).collect();
    let b: Vec<f32> = (0..sizes.1*sizes.2).into_iter().map(|_| rand::random()).collect();
    let c = monte::multiply_float(&a, &b, sizes, 500);
    println!("{:?}, {:?}", c, c.iter().sum::<f32>());
}


fn main() {
    println!("Hello, world!");
    short();
}



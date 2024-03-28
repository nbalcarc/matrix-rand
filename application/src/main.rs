use std::time::Instant;

use common::{monte, regular, tools::stats};
use rand;


#[derive(Debug, Clone)]
struct PerfBlock {
    name: String,
    time: f64,
    mae: f64,
}    


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
    println!("{:?}", monte::multiply(&a, &b, sizes, 3));
}


fn long() {
    let sizes = (2_048, 1_536, 1_024);
    let a: Vec<f32> = (0..sizes.0*sizes.1).into_iter().map(|_| rand::random()).collect();
    let b: Vec<f32> = (0..sizes.1*sizes.2).into_iter().map(|_| rand::random()).collect();
    let c = monte::multiply(&a, &b, sizes, 500);
    println!("{:?}, {:?}", c, c.iter().sum::<f32>());
}

fn thing() {
    let sizes = (2_048, 1_536, 1_024);
    let trials = 1;

    println!("Generating matrices");
    let a: Vec<f32> = (0..sizes.0*sizes.1).into_iter().map(|_| rand::random()).collect();
    let b: Vec<f32> = (0..sizes.1*sizes.2).into_iter().map(|_| rand::random()).collect();
    //let a_int: Vec<i32> = (0..sizes.0*sizes.1).into_iter().map(|_| rand::random()).collect();
    //let b_int: Vec<i32> = (0..sizes.1*sizes.2).into_iter().map(|_| rand::random()).collect();

    let mut c = Vec::new();

    let mut perf_blocks = Vec::with_capacity(16);

    // regular
    let test_name = String::from("regular");
    println!("Running test: {}", test_name);
    let mut times = Vec::with_capacity(trials);
    for i in 0..trials {
        println!("Iteration {}", i);
        let now = Instant::now();
        c = regular::multiply_float(&a, &b, sizes);
        times.push(now.elapsed());
    }
    perf_blocks.push(PerfBlock {
        name: test_name,
        time: times.into_iter().map(|x| x.as_secs_f64()).sum::<f64>() / trials as f64,
        mae: 0.0,
    });

    // regular int
    //let test_name = String::from("regular int");
    //println!("Running test: {}", test_name);
    //let mut times = Vec::with_capacity(trials);
    //for i in 0..trials {
    //    println!("Iteration {}", i);
    //    let now = Instant::now();
    //    regular::multiply_int(&a_int, &b_int, sizes);
    //    times.push(now.elapsed());
    //}
    //perf_blocks.push(PerfBlock {
    //    name: test_name,
    //    time: times.into_iter().map(|x| x.as_secs_f64()).sum::<f64>() / trials as f64,
    //    mae: 0.0,
    //});

    // monte carlo 1152
    let test_name = String::from("monte 1152");
    println!("Running test: {}", test_name);
    let mut times = Vec::with_capacity(trials);
    let mut accuracies = Vec::with_capacity(trials);
    for i in 0..trials {
        println!("Iteration {}", i);
        let now = Instant::now();
        let c_prime = monte::multiply(&a, &b, sizes, 1152);
        times.push(now.elapsed());
        accuracies.push(stats(&c, &c_prime));
    }
    perf_blocks.push(PerfBlock {
        name: test_name,
        time: times.into_iter().map(|x| x.as_secs_f64()).sum::<f64>() / trials as f64,
        mae: accuracies.into_iter().sum::<f64>() / trials as f64,
    });

    // monte carlo 768
    let test_name = String::from("monte 768");
    println!("Running test: {}", test_name);
    let mut times = Vec::with_capacity(trials);
    let mut accuracies = Vec::with_capacity(trials);
    for i in 0..trials {
        println!("Iteration {}", i);
        let now = Instant::now();
        let c_prime = monte::multiply(&a, &b, sizes, 768);
        times.push(now.elapsed());
        accuracies.push(stats(&c, &c_prime));
    }
    perf_blocks.push(PerfBlock {
        name: test_name,
        time: times.into_iter().map(|x| x.as_secs_f64()).sum::<f64>() / trials as f64,
        mae: accuracies.into_iter().sum::<f64>() / trials as f64,
    });

    // monte carlo 384
    let test_name = String::from("monte 384");
    println!("Running test: {}", test_name);
    let mut times = Vec::with_capacity(trials);
    let mut accuracies = Vec::with_capacity(trials);
    for i in 0..trials {
        println!("Iteration {}", i);
        let now = Instant::now();
        let c_prime = monte::multiply(&a, &b, sizes, 384);
        times.push(now.elapsed());
        accuracies.push(stats(&c, &c_prime));
    }
    perf_blocks.push(PerfBlock {
        name: test_name,
        time: times.into_iter().map(|x| x.as_secs_f64()).sum::<f64>() / trials as f64,
        mae: accuracies.into_iter().sum::<f64>() / trials as f64,
    });

    // monte carlo 192
    let test_name = String::from("monte 192");
    println!("Running test: {}", test_name);
    let mut times = Vec::with_capacity(trials);
    let mut accuracies = Vec::with_capacity(trials);
    for i in 0..trials {
        println!("Iteration {}", i);
        let now = Instant::now();
        let c_prime = monte::multiply(&a, &b, sizes, 192);
        times.push(now.elapsed());
        accuracies.push(stats(&c, &c_prime));
    }
    perf_blocks.push(PerfBlock {
        name: test_name,
        time: times.into_iter().map(|x| x.as_secs_f64()).sum::<f64>() / trials as f64,
        mae: accuracies.into_iter().sum::<f64>() / trials as f64,
    });



    println!("{:?}", perf_blocks);




}


fn main() {
    println!("Hello, world!");
    //short();
    thing();
}




//#[cfg(test)]
//mod tests {
//    use common::tools::stats;
//
//    use super::*;
//    use std::time::Instant;
//
//
//    #[derive(Debug, Clone)]
//    struct PerfBlock {
//        name: String,
//        time: f64,
//        mae: f64,
//    }    
//
//    #[test]
//    fn sayhi() {
//        println!("hi");
//    }
//
//    #[test]
//
//    //#[test]
//    //fn it_works() {
//    //    let result = add(2, 2);
//    //    assert_eq!(result, 4);
//    //}
//}



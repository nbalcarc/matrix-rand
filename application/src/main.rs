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


fn run_test(
    test_name: String,
    a: &[f32],
    b: &[f32],
    sizes: (usize, usize, usize),
    func: &dyn Fn(&[f32], &[f32], (usize, usize, usize)) -> Vec<f32>,
    c: &[f32],
    trials: usize,
) -> PerfBlock {

    println!("Running test: {}", test_name);
    let mut times = Vec::with_capacity(trials);
    let mut accuracies = Vec::with_capacity(trials);
    for i in 0..trials {
        println!("Iteration {}", i);
        let now = Instant::now();
        let c_prime = func(&a, &b, sizes);
        times.push(now.elapsed());
        accuracies.push(stats(&c, &c_prime));
    }
    
    PerfBlock {
        name: test_name,
        time: times.into_iter().map(|x| x.as_secs_f64()).sum::<f64>() / trials as f64,
        mae: accuracies.into_iter().sum::<f64>() / trials as f64,
    }
}


fn monte_1152(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
    monte::multiply(a, b, sizes, 1152)
}
fn monte_768(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
    monte::multiply(a, b, sizes, 768)
}
fn monte_384(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
    monte::multiply(a, b, sizes, 384)
}
fn monte_192(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
    monte::multiply(a, b, sizes, 192)
}


fn thing() {
    let sizes = (2_048, 1_536, 1_024);
    let trials = 10;

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
        //println!("{:?}", c);
        //println!("{:?}", c.iter().sum::<f32>() / c.len() as f32);
        //todo!();
        times.push(now.elapsed());
    }
    perf_blocks.push(PerfBlock {
        name: test_name,
        time: times.into_iter().map(|x| x.as_secs_f64()).sum::<f64>() / trials as f64,
        mae: 0.0,
    });

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



fn thing1() {
    let diseases = vec!["(vertigo) Paroymsal  Positional Vertigo", "AIDS", "Acne", "Alcoholic hepatitis", "Allergy", "Arthritis", "Bronchial Asthma", "Cervical spondylosis", "Chicken pox", "Chronic cholestasis", "Common Cold", "Dengue", "Diabetes ", "Dimorphic hemmorhoids(piles)", "Drug Reaction", "Fungal infection", "GERD", "Gastroenteritis", "Heart attack", "Hepatitis B", "Hepatitis C", "Hepatitis D", "Hepatitis E", "Hypertension ", "Hyperthyroidism", "Hypoglycemia", "Hypothyroidism", "Impetigo", "Jaundice", "Malaria", "Migraine", "Osteoarthristis", "Paralysis (brain hemorrhage)", "Peptic ulcer diseae", "Pneumonia", "Psoriasis", "Tuberculosis", "Typhoid", "Urinary tract infection", "Varicose veins", "hepatitis A"];
    let diseases_size = diseases.len();
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



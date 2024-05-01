mod csv_reader;


use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Instant};

use common::{countsketch::CountSketch, hadamard, monte::{self, MonteMultiplier}, neuralnet::NeuralNetwork, regular, tools::stats};
use csv_reader::{read_disease_test, read_disease_train};
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
    //println!("{:?}", regular::multiply_float(&a, &b, sizes));
    //println!("{:?}", monte::multiply(&a, &b, sizes, 3));
}

fn run_test(
    test_name: String,
    a: &[f32],
    b: &[f32],
    sizes: (usize, usize, usize),
    c: &mut [f32],
    func: &dyn Fn(&[f32], &[f32], (usize, usize, usize), &mut [f32]),
    y: &[f32],
    trials: usize,
) -> PerfBlock {

    println!("Running test: {}", test_name);
    let mut times = Vec::with_capacity(trials);
    let mut accuracies = Vec::with_capacity(trials);
    for i in 0..trials {
        println!("Iteration {}", i);
        let now = Instant::now();
        let y_prime = func(a, b, sizes, c);
        times.push(now.elapsed());
        accuracies.push(stats(&y, &y_prime));
    }
    
    PerfBlock {
        name: test_name,
        time: times.into_iter().map(|x| x.as_secs_f64()).sum::<f64>() / trials as f64,
        mae: accuracies.into_iter().sum::<f64>() / trials as f64,
    }
}


//fn monte_1152(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
//    monte::multiply(a, b, sizes, 1152)
//}
//fn monte_768(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
//    monte::multiply(a, b, sizes, 768)
//}
//fn monte_384(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
//    monte::multiply(a, b, sizes, 384)
//}
//fn monte_192(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
//    monte::multiply(a, b, sizes, 192)
//}
//fn hadamard_18432(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
//    hadamard::randomized_hadamard_transform(a, b, sizes, 18432)
//}
//fn hadamard_9216(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
//    hadamard::randomized_hadamard_transform(a, b, sizes, 9216)
//}
//fn hadamard_4608(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
//    hadamard::randomized_hadamard_transform(a, b, sizes, 4608)
//}
//fn hadamard_2304(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
//    hadamard::randomized_hadamard_transform(a, b, sizes, 2304)
//}
//fn hadamard_1536(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
//    hadamard::randomized_hadamard_transform(a, b, sizes, 1536)
//}
//fn hadamard_1152(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
//    hadamard::randomized_hadamard_transform(a, b, sizes, 1152)
//}
//fn hadamard_768(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
//    hadamard::randomized_hadamard_transform(a, b, sizes, 768)
//}
//fn hadamard_384(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
//    hadamard::randomized_hadamard_transform(a, b, sizes, 384)
//}
//fn hadamard_192(a: &[f32], b: &[f32], sizes: (usize, usize, usize)) -> Vec<f32> {
//    hadamard::randomized_hadamard_transform(a, b, sizes, 192)
//}


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

    //let countsketch_1536_3 = Rc::new(RefCell::new(CountSketch::new(sizes.0, 1536, 3)));
    //let countsketch_1536_10 = Rc::new(RefCell::new(CountSketch::new(sizes.0, 1536, 10)));
    //let countsketch_1152_3 = Rc::new(RefCell::new(CountSketch::new(sizes.0, 1152, 3)));
    //let countsketch_1152_10 = Rc::new(RefCell::new(CountSketch::new(sizes.0, 1152, 10)));
    //let countsketch_1024_3 = Rc::new(RefCell::new(CountSketch::new(sizes.0, 1024, 3)));
    //let countsketch_1024_10 = Rc::new(RefCell::new(CountSketch::new(sizes.0, 1024, 10)));

    //// regular
    //let test_name = String::from("regular");
    //println!("Running test: {}", test_name);
    //let mut times = Vec::with_capacity(trials);
    //for i in 0..1 { //CHANGE THIS BACK TO TRIALS
    //    println!("Iteration {}", i);
    //    let now = Instant::now();
    //    c = regular::multiply_float(&a, &b, sizes);
    //    times.push(now.elapsed());
    //}
    //perf_blocks.push(PerfBlock {
    //    name: test_name,
    //    time: times.into_iter().map(|x| x.as_secs_f64()).sum::<f64>() / trials as f64,
    //    mae: 0.0,
    //});

    //// monte carlo 1152
    //perf_blocks.push(run_test("monte 1152".to_string(), &a, &b, sizes, &monte_1152, &c, trials));

    //// monte carlo 768
    //perf_blocks.push(run_test("monte 768".to_string(), &a, &b, sizes, &monte_768, &c, trials));

    //// monte carlo 384
    //perf_blocks.push(run_test("monte 384".to_string(), &a, &b, sizes, &monte_384, &c, trials));

    //// monte carlo 192
    //perf_blocks.push(run_test("monte 192".to_string(), &a, &b, sizes, &monte_192, &c, trials));

    //// hadamard 1536
    //perf_blocks.push(run_test("hadamard 1536".to_string(), &a, &b, sizes, &hadamard_1536, &c, trials));

    //// hadamard 1152
    //perf_blocks.push(run_test("hadamard 1152".to_string(), &a, &b, sizes, &hadamard_1152, &c, trials));

    //// hadamard 768
    //perf_blocks.push(run_test("hadamard 768".to_string(), &a, &b, sizes, &hadamard_768, &c, trials));

    //// hadamard 384
    //perf_blocks.push(run_test("hadamard 384".to_string(), &a, &b, sizes, &hadamard_384, &c, trials));

    //// hadamard 192
    //perf_blocks.push(run_test("hadamard 192".to_string(), &a, &b, sizes, &hadamard_192, &c, trials));

    //// countsketch 1152 3
    //perf_blocks.push(run_test("countsketch 1152 3".to_string(), &a, &b, sizes, &(|a, b, sizes| {countsketch_1152_3.try_borrow_mut().unwrap().matrix_multiply(a, b, sizes)}), &c, trials));

    //// countsketch 1152 10
    //perf_blocks.push(run_test("countsketch 1152 10".to_string(), &a, &b, sizes, &(|a, b, sizes| {countsketch_1152_10.try_borrow_mut().unwrap().matrix_multiply(a, b, sizes)}), &c, trials));

    //// countsketch 1536 3
    //perf_blocks.push(run_test("countsketch 1536 3".to_string(), &a, &b, sizes, &(|a, b, sizes| {countsketch_1536_3.try_borrow_mut().unwrap().matrix_multiply(a, b, sizes)}), &c, trials));

    //// countsketch 1536 10
    //perf_blocks.push(run_test("countsketch 1536 10".to_string(), &a, &b, sizes, &(|a, b, sizes| {countsketch_1536_10.try_borrow_mut().unwrap().matrix_multiply(a, b, sizes)}), &c, trials));

    //// countsketch 1024 3
    //perf_blocks.push(run_test("countsketch 1024 3".to_string(), &a, &b, sizes, &(|a, b, sizes| {countsketch_1024_3.try_borrow_mut().unwrap().matrix_multiply(a, b, sizes)}), &c, trials));

    //// countsketch 1024 10
    //perf_blocks.push(run_test("countsketch 1024 10".to_string(), &a, &b, sizes, &(|a, b, sizes| {countsketch_1024_10.try_borrow_mut().unwrap().matrix_multiply(a, b, sizes)}), &c, trials));

    println!("{:?}", perf_blocks);
}



fn thing1() {
    let diseases = vec!["(vertigo) Paroymsal  Positional Vertigo", "AIDS", "Acne", "Alcoholic hepatitis", "Allergy", "Arthritis", "Bronchial Asthma", "Cervical spondylosis", "Chicken pox", "Chronic cholestasis", "Common Cold", "Dengue", "Diabetes ", "Dimorphic hemmorhoids(piles)", "Drug Reaction", "Fungal infection", "GERD", "Gastroenteritis", "Heart attack", "Hepatitis B", "Hepatitis C", "Hepatitis D", "Hepatitis E", "Hypertension ", "Hyperthyroidism", "Hypoglycemia", "Hypothyroidism", "Impetigo", "Jaundice", "Malaria", "Migraine", "Osteoarthristis", "Paralysis (brain hemorrhage)", "Peptic ulcer diseae", "Pneumonia", "Psoriasis", "Tuberculosis", "Typhoid", "Urinary tract infection", "Varicose veins", "hepatitis A"];
    let disease_ids: HashMap<String, usize> = diseases
        .iter()
        .enumerate()
        .map(|x| (String::from(*x.1), x.0))
        .collect();

    // get disease training data
    let disease_csv_raw = read_disease_train().unwrap();
    let disease_train_x: Vec<Vec<f32>> = disease_csv_raw.rows
        .iter()
        .map(|x| x.ints.iter().map(|x| *x as f32).collect())
        .collect();
    let disease_train_y: Vec<Vec<f32>> = disease_csv_raw.rows
        .iter()
        .map(|x| vec![disease_ids[&x.string] as f32])
        .collect();

    // get disease testing data
    let disease_csv_raw = read_disease_test().unwrap();
    let disease_test_x: Vec<Vec<f32>> = disease_csv_raw.rows
        .iter()
        .map(|x| x.ints.iter().map(|x| *x as f32).collect())
        .collect();
    let disease_test_y: Vec<Vec<f32>> = disease_csv_raw.rows
        .iter()
        .map(|x| vec![disease_ids[&x.string] as f32])
        .collect();

    println!("{:?}\n", disease_test_x);
    println!("{:?}\n", disease_test_y);
    println!("{}", diseases.len());
}


fn thing2() {
    let diseases = vec!["(vertigo) Paroymsal  Positional Vertigo", "AIDS", "Acne", "Alcoholic hepatitis", "Allergy", "Arthritis", "Bronchial Asthma", "Cervical spondylosis", "Chicken pox", "Chronic cholestasis", "Common Cold", "Dengue", "Diabetes ", "Dimorphic hemmorhoids(piles)", "Drug Reaction", "Fungal infection", "GERD", "Gastroenteritis", "Heart attack", "Hepatitis B", "Hepatitis C", "Hepatitis D", "Hepatitis E", "Hypertension ", "Hyperthyroidism", "Hypoglycemia", "Hypothyroidism", "Impetigo", "Jaundice", "Malaria", "Migraine", "Osteoarthristis", "Paralysis (brain hemorrhage)", "Peptic ulcer diseae", "Pneumonia", "Psoriasis", "Tuberculosis", "Typhoid", "Urinary tract infection", "Varicose veins", "hepatitis A"];
    let disease_ids: HashMap<String, usize> = diseases
        .iter()
        .enumerate()
        .map(|x| (String::from(*x.1), x.0))
        .collect(); //string to index

    // get disease training data
    let disease_csv_raw = read_disease_train().unwrap();
    let disease_train_x: Vec<Vec<f32>> = disease_csv_raw.rows
        .iter()
        .map(|x| x.ints.iter().map(|x| *x as f32).collect())
        .collect();
    let disease_train_y: Vec<Vec<f32>> = disease_csv_raw.rows
        .iter()
        .map(|x| {
            let mut y = vec![0.0; diseases.len()];
            y[disease_ids[&x.string]] = 1.0;
            y
        })
        .collect();

    // get disease testing data
    let disease_csv_raw = read_disease_test().unwrap();
    let disease_test_x: Vec<Vec<f32>> = disease_csv_raw.rows
        .iter()
        .map(|x| x.ints.iter().map(|x| *x as f32).collect())
        .collect();
    let disease_test_y: Vec<Vec<f32>> = disease_csv_raw.rows
        .iter()
        .map(|x| {
            let mut y = vec![0.0; diseases.len()];
            y[disease_ids[&x.string]] = 1.0;
            y
        })
        .collect();

    println!("{:?}\n", disease_test_x);
    println!("{:?}\n", disease_test_y);
    println!("{}", diseases.len());
    println!("{}", disease_test_x.len());
    println!("{}", disease_train_x.len());
}


fn main() {
    println!("Hello, world!");
    thing2();
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



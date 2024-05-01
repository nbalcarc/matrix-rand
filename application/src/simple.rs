use std::time::Instant;
use common::{countsketch::CountSketch, hadamard::HadamardMultiplier, monte::MonteMultiplier, neuralnet::Multiplier, regular, tools::stats};


#[derive(Debug, Clone)]
struct PerfBlock {
    name: String,
    time: f64,
    mae: f64,
}    


fn run_test<T: Multiplier>(
    test_name: String,
    a: &[f32],
    b: &[f32],
    sizes: (usize, usize, usize),
    c: &mut [f32],
    multiplier: &mut T, 
    y: &[f32],
    trials: usize,
) -> PerfBlock {

    println!("Running test: {}", test_name);
    let mut times = Vec::with_capacity(trials);
    let mut accuracies = Vec::with_capacity(trials);
    for i in 0..trials {
        println!("Iteration {}", i);
        let now = Instant::now();
        multiplier.multiply(a, b, sizes, c);
        times.push(now.elapsed());
        accuracies.push(stats(y, c));
    }
    
    PerfBlock {
        name: test_name,
        time: times.into_iter().map(|x| x.as_secs_f64()).sum::<f64>() / trials as f64,
        mae: accuracies.into_iter().sum::<f64>() / trials as f64,
    }
}


pub fn run_simple() {
    let sizes = (2_048, 1_536, 1_024);
    let trials = 10;

    println!("Generating matrices");
    let a: Vec<f32> = (0..sizes.0*sizes.1).into_iter().map(|_| rand::random()).collect();
    let b: Vec<f32> = (0..sizes.1*sizes.2).into_iter().map(|_| rand::random()).collect();

    let mut c = vec![0.0; sizes.0 * sizes.2];
    let mut perf_blocks = Vec::with_capacity(17);

    // regular
    let test_name = String::from("regular");
    println!("Running test: {}", test_name);
    let mut times = Vec::with_capacity(trials);
    for i in 0..trials { //CHANGE THIS BACK TO TRIALS
        println!("Iteration {}", i);
        let now = Instant::now();
        regular::multiply(&a, &b, sizes, &mut c);
        times.push(now.elapsed());
    }
    perf_blocks.push(PerfBlock {
        name: test_name,
        time: times.into_iter().map(|x| x.as_secs_f64()).sum::<f64>() / trials as f64,
        mae: 0.0,
    });

    // save reference for rest of multiplications
    let mut y = c.clone();

    // monte carlo 1536
    perf_blocks.push(run_test("monte 1536".to_string(), &a, &b, sizes, &mut c, &mut MonteMultiplier::new(1536), &y, trials));

    // monte carlo 1152
    perf_blocks.push(run_test("monte 1152".to_string(), &a, &b, sizes, &mut c, &mut MonteMultiplier::new(1152), &y, trials));

    // monte carlo 768
    perf_blocks.push(run_test("monte 768".to_string(), &a, &b, sizes, &mut c, &mut MonteMultiplier::new(768), &y, trials));

    // monte carlo 384
    perf_blocks.push(run_test("monte 384".to_string(), &a, &b, sizes, &mut c, &mut MonteMultiplier::new(384), &y, trials));

    // monte carlo 192
    perf_blocks.push(run_test("monte 192".to_string(), &a, &b, sizes, &mut c, &mut MonteMultiplier::new(192), &y, trials));

    // hadamard 1536
    perf_blocks.push(run_test("hadamard 1536".to_string(), &a, &b, sizes, &mut c, &mut HadamardMultiplier::new(1536), &y, trials));

    // hadamard 1152
    perf_blocks.push(run_test("hadamard 1152".to_string(), &a, &b, sizes, &mut c, &mut HadamardMultiplier::new(1152), &y, trials));

    // hadamard 768
    perf_blocks.push(run_test("hadamard 768".to_string(), &a, &b, sizes, &mut c, &mut HadamardMultiplier::new(768), &y, trials));

    // hadamard 384
    perf_blocks.push(run_test("hadamard 384".to_string(), &a, &b, sizes, &mut c, &mut HadamardMultiplier::new(384), &y, trials));

    // hadamard 192
    perf_blocks.push(run_test("hadamard 192".to_string(), &a, &b, sizes, &mut c, &mut HadamardMultiplier::new(192), &y, trials));

    // countsketch 1536 3
    perf_blocks.push(run_test("countsketch 1536 3".to_string(), &a, &b, sizes, &mut c, &mut CountSketch::new(1536, 3), &y, trials));

    // countsketch 1536 10
    perf_blocks.push(run_test("countsketch 1536 10".to_string(), &a, &b, sizes, &mut c, &mut CountSketch::new(1536, 10), &y, trials));

    // countsketch 1152 3
    perf_blocks.push(run_test("countsketch 1152 3".to_string(), &a, &b, sizes, &mut c, &mut CountSketch::new(1152, 3), &y, trials));

    // countsketch 1152 10
    perf_blocks.push(run_test("countsketch 1152 10".to_string(), &a, &b, sizes, &mut c, &mut CountSketch::new(1152, 10), &y, trials));

    // countsketch 1024 3
    perf_blocks.push(run_test("countsketch 1024 3".to_string(), &a, &b, sizes, &mut c, &mut CountSketch::new(1024, 3), &y, trials));

    // countsketch 1024 10
    perf_blocks.push(run_test("countsketch 1024 10".to_string(), &a, &b, sizes, &mut c, &mut CountSketch::new(1024, 10), &y, trials));

    println!("{:?}", perf_blocks);


}




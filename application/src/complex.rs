use std::{collections::HashMap, fs, io::Write, time::Instant};
use common::{hadamard::HadamardMultiplier, monte::MonteMultiplier, neuralnet::{Multiplier, NeuralNetwork}, regular::RegularMultiplier};
use rand::{seq::SliceRandom, thread_rng};

use crate::csv_reader::{read_disease_test, read_disease_train};


fn setup() -> (Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<Vec<f32>>) {
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

    (disease_train_x, disease_train_y, disease_test_x, disease_test_y)
}


pub fn train_model<T: Multiplier>(
    name: String,
    multiplier: T,
    training_data: &Vec<(Vec<f32>, Vec<f32>)>,
    testing_data: &Vec<(Vec<f32>, Vec<f32>)>,
) -> (String, f64, Vec<f32>, f32) {
    println!("Now training model with: {}", &name);
    let epochs = 100;
    let mut eta = 1.0; //learning rate

    //let (disease_train_x, disease_train_y, disease_test_x, disease_test_y) = setup(); //4920 training
    //let mut zipped: Vec<(Vec<f32>, Vec<f32>)> = disease_train_x.into_iter()
    //    .zip(disease_train_y.into_iter())
    //    .collect();
    let mut data = training_data.clone();
    let mut rng = thread_rng();
    //let mut neuralnet = NeuralNetwork::new(RegularMultiplier::new(), (132, 260, 120, 41));
    let mut neuralnet = NeuralNetwork::new(multiplier, (132, 260, 120, 41));
    let mut training_errors = Vec::with_capacity(41 * epochs);

    let now = Instant::now();
    for i in 0..epochs {
        println!("Running epoch #{}", i + 1);

        // shuffle and chunkify
        data.shuffle(&mut rng);
        let batches: Vec<&[(Vec<f32>, Vec<f32>)]> = data.chunks(492).collect(); //120


        for batch in batches {
            let errors = neuralnet.train(batch, eta);
            let batch_err = errors.iter().sum::<f32>() / errors.len() as f32;
            training_errors.push(batch_err);

        }
        eta -= 1.0 / epochs as f32;

        //println!("Epoch error: {}", all_errors[all_errors.len() - 1]);
    }
    let time = now.elapsed().as_secs_f64();

    // run testing phase
    let mut tally = 0;
    for (x, y) in testing_data {
        let y_prime = neuralnet.forward(x);
        let mut correct = true;

        // compare
        for i in 0..y.len() {
            if y_prime[i].round() != y[i] {
                correct = false;
            }
        }

        if correct {
            tally += 1;
        }
    }

    (name, time, training_errors, tally as f32 / testing_data.len() as f32)
}


pub fn run_complex_disease() {
    let (disease_train_x, disease_train_y, disease_test_x, disease_test_y) = setup(); //4920 training
    let training_data: Vec<(Vec<f32>, Vec<f32>)> = disease_train_x.into_iter()
        .zip(disease_train_y.into_iter())
        .collect();
    let testing_data: Vec<(Vec<f32>, Vec<f32>)> = disease_test_x.into_iter()
        .zip(disease_test_y.into_iter())
        .collect();

    let mut results = Vec::with_capacity(17);
    results.push(train_model("regular".to_string(), RegularMultiplier::new(), &training_data, &testing_data));
    results.push(train_model("monte 1536".to_string(), MonteMultiplier::new(1536), &training_data, &testing_data));
    results.push(train_model("monte 1152".to_string(), MonteMultiplier::new(1152), &training_data, &testing_data));
    results.push(train_model("monte 768".to_string(), MonteMultiplier::new(768), &training_data, &testing_data));
    results.push(train_model("monte 384".to_string(), MonteMultiplier::new(384), &training_data, &testing_data));
    results.push(train_model("monte 192".to_string(), MonteMultiplier::new(192), &training_data, &testing_data));
    results.push(train_model("monte 96".to_string(), MonteMultiplier::new(96), &training_data, &testing_data));
    results.push(train_model("monte 48".to_string(), MonteMultiplier::new(48), &training_data, &testing_data));
    results.push(train_model("monte 24".to_string(), MonteMultiplier::new(24), &training_data, &testing_data));
    results.push(train_model("hadamard 1536".to_string(), HadamardMultiplier::new(1536), &training_data, &testing_data));
    results.push(train_model("hadamard 1152".to_string(), HadamardMultiplier::new(1152), &training_data, &testing_data));
    results.push(train_model("hadamard 768".to_string(), HadamardMultiplier::new(768), &training_data, &testing_data));
    results.push(train_model("hadamard 384".to_string(), HadamardMultiplier::new(384), &training_data, &testing_data));
    results.push(train_model("hadamard 192".to_string(), HadamardMultiplier::new(192), &training_data, &testing_data));
    let encoded = serde_pickle::to_vec(&results, Default::default()).unwrap();
    let mut file = fs::OpenOptions::new().create(true).write(true).open("./data_output.pkl").unwrap();
    file.write_all(&encoded).unwrap();
}



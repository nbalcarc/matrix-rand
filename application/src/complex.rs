use std::collections::HashMap;
use common::{neuralnet::NeuralNetwork, regular::RegularMultiplier};
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


pub fn run_complex_disease() {
    let epochs = 100;
    let mut eta = 1.0; //learning rate

    let (disease_train_x, disease_train_y, disease_test_x, disease_test_y) = setup(); //4920 training
    let mut zipped: Vec<(Vec<f32>, Vec<f32>)> = disease_train_x.into_iter()
        .zip(disease_train_y.into_iter())
        .collect();
    let mut rng = thread_rng();
    let mut neuralnet = NeuralNetwork::new(RegularMultiplier::new(), (132, 260, 120, 41));
    let mut all_errors = Vec::with_capacity(41 * epochs);

    for i in 0..epochs {
        println!("Running epoch #{}", i + 1);

        // shuffle and chunkify
        zipped.shuffle(&mut rng);
        let batches: Vec<&[(Vec<f32>, Vec<f32>)]> = zipped.chunks(492).collect(); //120


        for batch in batches {
            let errors = neuralnet.train(batch, eta);
            let batch_err = errors.iter().sum::<f32>() / errors.len() as f32;
            all_errors.push(batch_err);

        }
        eta -= 1.0 / epochs as f32;

        //println!("Epoch error: {}", all_errors[all_errors.len() - 1]);
    }

    println!("{:?}", all_errors);

    //println!("{:?}\n", disease_test_x);
    //println!("{:?}\n", disease_test_y);
    //println!("{}", disease_test_x.len());
    //println!("{}", disease_train_x.len());


}



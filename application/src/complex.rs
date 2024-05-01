

fn run_complex_disease() {
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

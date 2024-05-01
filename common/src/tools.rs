pub struct Stats {
    /// mean absolute error
    pub mae: f64,
}


pub fn stats(c: &[f32], c_prime: &[f32]) -> f64 {
    c.iter().zip(c_prime).map(|(x, y)| (x - y).abs() as f64).sum::<f64>() / c.len() as f64
}


pub fn normalize_vector(a: &mut [f32]) {
    let mean = a.iter().sum::<f32>() / a.len() as f32;
    let variance = a.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / a.len() as f32;
    for i in 0..a.len() {
        a[i] = (a[i] - mean) / (variance + 0.00000001).sqrt();
    }
}



pub struct Stats {
    /// mean absolute error
    pub mae: f64,
}

pub fn stats(c: &[f32], c_prime: &[f32]) -> f64 {
    c.iter().zip(c_prime).map(|(x, y)| (x - y).abs() as f64).sum::<f64>() / c.len() as f64
}



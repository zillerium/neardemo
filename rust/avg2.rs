fn find_average(xs: &[f64]) -> f64 {
    match xs.len() {
        0 => 0.,
        n => xs.iter().sum::<f64>() / n as f64
    }
}

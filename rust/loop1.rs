fn generate_range(min: usize, max: usize, step: usize) -> Vec<usize> {
    (min..=max).step_by(step).collect()
}

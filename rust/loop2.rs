fn generate_range(min: usize, max: usize, step: usize) -> Vec<usize> {
    let mut vec = Vec::new();
    
    for i in (min..max+1).step_by(step) {
        vec.push(i);
    }
    
    vec
}

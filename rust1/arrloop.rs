fn arr(n: usize) -> Vec<u32> {
    // the numbers 0 to n-1
    let mut v = Vec::new();
    for i in 0..n {
        v.push(i as u32);
    }
    v
}

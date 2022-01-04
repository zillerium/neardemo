fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    (n..=limit).filter(|v| v % n == 0).collect()
}

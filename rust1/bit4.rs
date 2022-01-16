fn extra_perfect(n: u32) -> Vec<u32> {
    (1..n+1).filter(|&x| x % 2 == 1).collect()
}

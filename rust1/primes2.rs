fn num_primorial(n: usize) -> u64 {
    (2..)
        .filter(|x| is_prime(*x))
        .map(|x| x as u64)
        .take(n)
        .product::<u64>()
}

fn is_prime(n: u32) -> bool {
    for a in 2..n {
        if n % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}

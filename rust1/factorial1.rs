fn strong(n: u64) -> String {
    let mut s = 0;
    let mut t = n;
    while t > 0 {
        s += factorial(t % 10);
        t /= 10;
    }
    return match s == n {
        true => String::from("STRONG!!!!"),
        false => String::from("Not Strong !!"),
    }
}

fn factorial(n: u64) -> u64 {
    let mut p = 1;
    for i in 1..n+1 {
        p *= i;
    }
    p
}

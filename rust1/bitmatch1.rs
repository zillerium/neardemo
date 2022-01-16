fn add(a: i32, b: i32) -> i32 {
    match a & b {
        0 => a | b,
        c => add(a ^ b, c << 1)
    }
}

fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    *[a + b + c, a * (b + c), (a + b) * c, a * b * c].iter().max().unwrap()
}

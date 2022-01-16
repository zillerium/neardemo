fn tidy_number(n: u64) -> bool {
    n.to_string().as_bytes().windows(2).all(|ds| ds[0] <= ds[1])
}

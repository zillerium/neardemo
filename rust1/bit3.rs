fn is_perfect(n: &u32) -> bool {
    let bitstring = format!("{:b}", n);
    bitstring.starts_with("1") & bitstring.ends_with("1")
}

fn extra_perfect(n: u32) -> Vec<u32> {
    (1..=n).filter(is_perfect).collect()
}

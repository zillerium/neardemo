fn reverse_seq(n: u32) -> Vec<u32> {
    //unimplemented!();
    let mut t = Vec::new();
    for i in 1..n+1 {
        t.push(n+1-i);
    }
    t
}

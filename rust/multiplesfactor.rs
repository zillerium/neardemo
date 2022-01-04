fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
   // todo!()
 
    let mut vec = Vec::new();
    let mut tot = 0;
    while tot < limit {
        tot += n;
        if (tot <= limit) {
            vec.push(tot)
        }
    }
    vec
}


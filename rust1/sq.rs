n automorphic(n: u64) -> String {
  //  todo!()#
    let char_vec: Vec<char> = n.to_string().chars().collect();
    let l = char_vec.len() as u32;
    let n2 = n * n;
    let p = i32::pow(10, l) as u64;
    let rem = n2 % p;
    if rem == n {
        "Automorphic".to_string()
    } else {
        "Not!!".to_string()
    }
}

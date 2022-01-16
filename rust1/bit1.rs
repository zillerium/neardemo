fn extra_perfect(n: u32) -> Vec<u32> {
 //   todo!()
    let mut a: Vec<u32> = Vec::new();
    for i in 0..n+1 {
        if test(i) {
            a.push(i);
        }
        
    }
    a
}
    
fn test (n:u32)-> bool {
    let g = format!("{:b}", n);
    let bin_v: Vec<char> = g.to_string().chars().collect();
    let l = bin_v.len()-1;
      println!("{} n==", n);
    println!("{}", bin_v[0]);
    println!("{}", bin_v[l]);
    if bin_v[0] == '1' && bin_v[l] == '1' {
        true
    } else {
        false
    }
}


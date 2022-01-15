fn balanced_num(n: u64) -> String {
    //todo!()
    println!("{} n - ", n);
    let mut str = n.to_string();
    if n < 100 {
        str = ["0", &str, "0"].join("");
    }
    println!("{} string - ", str);
      let char_vec: Vec<char> = str.chars().collect();

//let my_number = usize::from_str_radix(&n.to_string(), 10);

    let len_n:u32 = char_vec.len() as u32;
    let mut a: Vec<char> = Vec::new();
    let mut b: Vec<char> = Vec::new();
    let   s1:u32;
    let   s2:u32;
    let m = len_n / 2;
    
    if len_n % 2 == 0 {
        s1 = m-1;
        s2 = m+1;
    } else {
        s1 = m;
        s2 = m+1;
    }
    
        // m - 1let mut a = Vec:new();
        for i in 0..s1 {
            let y = char_vec[i as usize] as char;
            a.push(y);        
        }
        for i in s2..len_n {
            let y = char_vec[i as usize] as char;
             b.push(y);
        }
       let mut tot = 0;
    let mut tot1 = 0;
       for ch in &a {
 // println!("{} {}", i, ch);
            tot =tot + (*ch as i32);
        }
       for ch in &b {
 // println!("{} {}", i, ch);
  tot1 =tot1 + (*ch as i32);
}
        if tot == tot1 {
            "Balanced".to_string()
        } else {
            "Not Balanced".to_string()
        }
   
}

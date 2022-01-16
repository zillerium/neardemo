fn special_number(n: u64) -> String {
//    todo!()
     let   str = n.to_string();
    println!("{}", str);
       const RADIX: u32 = 10;
  
      let mut failed = false;
      let char_vec: Vec<char> = str.chars().collect();
    for  i in &char_vec {
    
        let y = i.to_digit(RADIX).unwrap() as u32;
        println!("{}", y);
        if y > 5 {
            failed = true;
            break;
        }
        
    }

    if failed {
        "NOT!!".to_string()
    } else {
        "Special!!".to_string()
    }
}

n tidy_number(n: u64) -> bool {
    let a: Vec<char> = n.to_string().chars().collect();
    
      const RADIX: u32 = 10;
      let mut failed = false;
      let mut prev = a[0].to_digit(RADIX).unwrap() as u64;
           for ch in &a {
 // println!("{} {}", i, ch);
            let x = ch.to_digit(RADIX).unwrap() as u64;
            println!("{} x ", x);
               println!("{} prev ", prev);
            if x>= prev {
                prev = x;
            } else {
                failed = true;
               break;
            }
             
        }

    
 !failed
 
}

n jumping_number(n: u64) -> String {
  //  todo!()
  // len = 4, len/2 = 2, m-1, m
    println!("{} n", n);
    let mut t = n;
    let mut prev = t % 10;
        t /= 10;
    let mut failed = false;
    while t > 0 {
        let x = t % 10;
        let mut diff = 0;
        if prev < x {
            diff = x - prev;
        } else {
            diff = prev - x;
        }
        
     
         println!("{} diff ", diff);
        if diff != 1 {
            failed = true;
        }
        prev = x;
        println!("{} x", x);
        t /= 10;
    }
    if failed {
        "Not!!".to_string()
        
    } else {
         "Jumping!!".to_string()
    }
}
         
    

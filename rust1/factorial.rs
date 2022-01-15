fn strong(n: u64) -> String {
    //todo!()
 
 // len = 4, len/2 = 2, m-1, m
     let char_vec: Vec<char> = n.to_string().chars().collect();
         let mut a: Vec<char> = Vec::new();
  

     let len_n:u32 = char_vec.len() as u32;
        for i in 0..len_n {
            let y = char_vec[i as usize] as char;
            a.push(y);        
        }
     println!("{:?}", a);
     let  mut tot = 0;
      const RADIX: u32 = 10;
           for ch in &a {
 // println!("{} {}", i, ch);
            let x = ch.to_digit(RADIX).unwrap() as u64;
            println!("{} x=", x);
             println!("{} ch=", *ch);
             
            let xf =factorial(x);
             println!("{} xf=", xf);
            tot += xf;
        }

println!("{} tot =", tot);
  //  for (i,ch) in a.iter().enumerate() {
   
 // numbers are 100, 10, 1
 // determine size of number 
    if tot == n {
        "STRONG!!!!".to_string()
    } else {
        "Not Strong !!".to_string()
    }

}

fn factorial(num: u64) -> u64 {
       match num {
        0  => 1,
        1.. => (1..num+1).product(),
    }
} 
 


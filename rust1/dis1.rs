fn disarium_number(n: u32) -> String {
  //  todo!()
     //0,1,2,3
 // len = 4, len/2 = 2, m-1, m
     let char_vec: Vec<char> = n.to_string().chars().collect();
     let len_n:u32 = char_vec.len() as u32;

    let mut t = n;
    let mut s = 0;
    let mut count = len_n+1;
    while t > 0 {
        count -=1;
        let x = t % 10;
        s += u32::pow(x, count);
        t /= 10;
    }
    println!("{} ", s);
  if s == n {
       "Disarium !!".to_string()
    } else {
        "Not !!".to_string()       
    }         
    
}  
 

       

         
    

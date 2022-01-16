 //  todo!()

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}

fn num_primorial(n: usize) -> u64 { 
    println!("{} n ", n);
    let mut x = n as u64;  
    let mut a: Vec<u64> = Vec::new();
    let mut y:u64 = 1;
    while x!=0 {
        println!("{} x", x);
        if is_prime(y) {
            println!("prime ");
            a.push(y);
             x-=1;
        }
       y+=1;
        
    }
  
    
    println!("{:?}", a);
    let b =  a.iter().fold(1,|a,b| a*b);
    
    println!("{:?}", b);
    b
    
}


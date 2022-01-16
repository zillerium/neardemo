fn main() { 
 let n= 23; //0,1,2,3
 // len = 4, len/2 = 2, m-1, m
    let mut t = n;
    let mut prev = t % 10;
    let mut failed = false;
    while t > 0 {
        let x = t % 10;
        let diff:i32 = prev - x;
         println!("{} doff ", diff);
        if diff.abs() != 1 {
            failed = true;
        }
        prev = x;
        println!("{} x", x);
        t /= 10;
    }
    println!("{} ", failed);
         
    
}


fn main() {
    
    
     println!("{}", positive_sum(&[1,2,3,4,5]))
}

fn positive_sum(slice: &[i32]) -> i32 {
    // your code
    let mut tot = 0;
    for i in slice {
        if *i > 0 {
            tot += i;
        }
        
    
    } 
    tot
}

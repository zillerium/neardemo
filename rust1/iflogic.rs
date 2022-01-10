fn count_red_beads(n: u32) -> u32 {
    //..
    println!("{}", n);
   
    if n <= 1 {
        0
    } else {
        (n-1) * 2
    }
    
}

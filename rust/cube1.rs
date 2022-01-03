fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    
    ((a.iter().fold(1i32, |x1, &y| x1 * y))
    - (b.iter().fold(1i32, |x1, &y| x1 * y))).abs()
         
}

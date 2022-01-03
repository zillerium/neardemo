fn main() {
 
    let some_numbers = vec![1,3,4,5];
    println!("{}", some_numbers.iter().fold(0, |x, y| x + y))
   
 
}

fn get_age(age: &str) -> u32 {
  // Your code here
    const RADIX: u32 = 10;
    
    
     let ch = age.chars().nth(0).unwrap();
     //let ch1 = "1";
     return ch.to_string().chars().map(|c| c.to_digit(RADIX).unwrap()).sum::<u32>();
 

} 

fn main() {
 
   println!("{:?}", get_age("1 year"));
 
}

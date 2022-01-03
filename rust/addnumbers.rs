 
fn main() {
 
 let n = 10;
  
  println!(
  "{}",
   (1..=n).fold(0, |sum, item| sum + item)
  
  );
   
 
}

 
fn main() {
 
  let a_string = "this is my example";
  
  println!(
  "{}",
  a_string.chars().fold("-".to_string(), |mut x, y| {
      x.push(y);
      x.push('-');
      x
  })
  
  );
   
 
}

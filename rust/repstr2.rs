fn repeat_str(src: &str, count: usize) -> String {
  let mut str = String::new();
  for _i in 0..count  {
      str.push_str(src)
  }
  return str;
}
 
fn main() {
   println!("{}",repeat_str("hello", 5))
}

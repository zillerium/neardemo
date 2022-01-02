fn greet(name: &str) -> String {
let   t: String = "Hello, ".to_string();
let   t1: String = " how are you today?".to_string();
let hello: String = t + &name + &t1;
 
 return hello;
 
    
    
}
fn main() {
 
   println!("{:?}", greet("Trevor"));
}

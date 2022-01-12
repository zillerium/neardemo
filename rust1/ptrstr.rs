fn main() {

let mut message = String::from("hello");
let message2: &mut String = &mut message;
message2.push_str("trevor");
println!("{}", message);
println!("{}", message2);


}

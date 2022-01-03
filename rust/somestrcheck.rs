fn main() {
 
let name = String::from("trevor");

println!("char at index 8:{}", match name.chars().nth(8) {
    Some(c) => c.to_string(),
    None => "failed".to_string()
})
   
 
}

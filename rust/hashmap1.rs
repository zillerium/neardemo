use std::collections::HashMap;



fn main() {
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), String::from("trevor"));
scores.insert(String::from("Blue1"), String::from("trevor1"));

let team_name = String::from("Blue1");
let score = scores.get(&team_name);
println!("{:?}", score);


} 
        
        


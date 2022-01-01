 
fn main() {
    
    
     println!("{}",  remove_char("eloquent"))
}

pub fn remove_char(s: &str) -> String {

    // Your code here!
    let slice = &s[1..s.len()-1];
    String::from(slice)
}

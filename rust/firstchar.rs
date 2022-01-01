fn main() {
    
    
     println!("{}",  remove_char("eloquent"))
}

pub fn remove_char(s: &str) -> String {

    // Your code here!
    let ch = s.chars().nth(0).unwrap();
    println!("{}", ch);
    String::from(s)
}

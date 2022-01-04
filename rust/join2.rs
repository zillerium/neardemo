fn reverse_words(words: &str) -> String {
   // "backward! is This".to_string()
    //words.chars().rev().collect::<String>()
    let test: Vec<&str> = words.split(' ').rev().collect();
    
     println!("{:?}", test.join(" "));
     "hh".to_string()
     
}


 
fn main() {

   println!("{:?}", reverse_words("this is a check"));
}

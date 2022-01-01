use std::iter::FromIterator;
fn descending_order(x: u64) -> u64 {
    let mut result = x.to_string().chars().collect::<Vec<char>>();
    result.sort_by(|a, b| b.cmp(a));
    String::from_iter(result).parse::<u64>().unwrap()
   
}

fn main() {
    
    
     println!("{}", descending_order(120881))
}

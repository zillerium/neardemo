use std::collections::BTreeSet;
 
fn main() {
 
let mut favorites = BTreeSet::new();
favorites.insert("Lucy in the Sky with Diamonds".to_string());
favorites.insert("Trevor".to_string());
   

let mut it = favorites.into_iter();
assert_eq!(it.next(), Some("Trevor".to_string()))
 
}

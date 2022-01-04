 fn two_sort(_arr: &[&str]) -> String {
    // your code here
    let  mut vec: Vec<String> = vec!["JJJ".to_string(), "AAA".to_string()];
      let   vec1 = vec.sort();
    println!("{:?})",vec1);
  
    "jj".to_string()
}

fn main() {
    println!("{:?}",two_sort(&["bitcoin", "take", "over", "the", "world"]));
    
}

fn warn_the_sheep(queue: &[&str]) -> String {
   // todo!();
    let needle: String = "wolf".to_string();
    let haystack = queue.to_vec();
    let N = 1;
    
    if let Some(str) = haystack.iter().find(|&s| *s == needle) {
        println!("Pls go away and stop eating my sheep");
    } else {
        println!("Oi! Sheep number {}! You are about to be eaten by a wolf!", N.to_string());
    }
}

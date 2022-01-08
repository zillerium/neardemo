fn count_sheep(n: u32) -> String {
    // your code here
    if n == 0 {return "".to_string()};
    let mut a = String::new();
    for i in 1..n+1 {
        let y = format!("{} sheep...", i.to_string());
            
        a.push_str(&y);
    }
    return a;
}

fn multi_table(n: u64) -> String {
    let mut str = String::new();
    for i in 1..11 {
        let tot = i * n;
    //    let line = i.to_string() + " * ".to_string() + n.to_string() 
    //    + " = " + tot.to_string() + "\n".to.string();
        if i == 10 {
           let line = format!("{} * {} = {}", i, n, tot); 
            str.push_str(&line);
        } else {
            let lineq = format!("{} * {} = {}\n", i, n, tot); 
            str.push_str(&lineq);
        }
    }    
    return str;
}
  

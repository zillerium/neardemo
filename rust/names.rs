fn hello(name: &str) -> String {
    if name.to_string().is_empty() { return format!("Hello, World!")} else {
        let newname = name.to_string().to_lowercase();
        let mut v: Vec<char> = newname.chars().collect(); 
        v[0]=v[0].to_uppercase().nth(0).unwrap();
        let s2: String = v.into_iter().collect();
    return format!("Hello, {}!", s2); }

   // "Hello, ".to_string() + name.to_string() + "!".to_string()
}

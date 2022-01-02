fn greet(name: &str) -> String {
    let mut hello = String::from("Hello, ");
        hello.push_str(name) ;
        hello.push_str(" how are you doing today?") ;
    return hello;  
}

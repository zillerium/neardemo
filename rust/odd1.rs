fn even_or_odd(i: i32) -> &'static str {
    if i % 2 == 0 {
        return "Even";
    } else {
        return "Odd"; 
    }
}

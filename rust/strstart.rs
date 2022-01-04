fn feast(beast: &str, dish: &str) -> bool {
    // your code here
    let s = beast.chars().nth(0).unwrap();
    let e = beast.chars().nth(beast.len()-1).unwrap();
    let s1 = dish.chars().nth(0).unwrap();
    let e1 = dish.chars().nth(dish.len()-1).unwrap();
    if s == s1 && e == e1 { return true;} else {return false;}
}

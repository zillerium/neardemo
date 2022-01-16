fn special_number(n: u64) -> String {
    if n.to_string().chars().any(|x| x.to_digit(10).unwrap() > 5) {
        String::from("NOT!!")
    } else {
        String::from("Special!!")
    }
}

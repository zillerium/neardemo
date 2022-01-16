fn special_number(n: u64) -> String {
    let specialNumbers = "012345";
    if n.to_string().chars().all(|x| specialNumbers.contains(x)) {
        "Special!!".to_string()
    } else {
        "NOT!!".to_string()
    }
}

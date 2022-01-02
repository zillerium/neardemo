fn abbrev_name(name: &str) -> String {
    let mut names = name.split(" ");
    let first = names.next().unwrap();
    let second = names.next().unwrap();
    return first[0..1].to_uppercase().to_string() + "." + &second[0..1].to_uppercase();
}

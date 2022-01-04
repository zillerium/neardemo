fn are_you_playing_banjo(name: &str) -> String {
    //todo!()
    if name.chars().nth(0).unwrap() == 'R' ||
    name.chars().nth(0).unwrap() == 'r' {
        name.to_string() + " plays banjo"
    } else {
        name.to_string() + " does not play banjo"
    }
}

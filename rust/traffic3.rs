fn update_light(current: &str) -> String {
    ["green", "yellow", "red"]
        .iter()
        .cycle()
        .skip_while(|&x| *x != current)
        .skip(1)
        .next()
        .unwrap()
        .to_string()
}

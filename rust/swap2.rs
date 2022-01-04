fn name_shuffler(s: &str) -> String {
    s.rsplit(" ").collect::<Vec<&str>>().join(" ")
}

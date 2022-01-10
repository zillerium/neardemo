fn get_count(string: &str) -> usize {
    string.matches(|x| match x {'a'|'e'|'i'|'o'|'u' => true, _ => false}).count()
}

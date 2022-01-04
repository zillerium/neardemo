fn reverse_words(str: &str) -> String {
   return str.split_whitespace()
        .rev()
        .collect::<Vec<_>>()
        .join(" ");
}

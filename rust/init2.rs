fn abbrev_name(name: &str) -> String {
  name.split(' ')
      .map(|x| x.chars().nth(0).unwrap().to_string().to_uppercase())
      .collect::<Vec<_>>()
      .join(".")
}

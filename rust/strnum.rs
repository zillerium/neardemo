fn string_to_number(s: &str) -> i32 {
  let my_string = s.to_string();  // `parse()` works with `&str` and `String`!
let my_int = my_string.parse::<i32>().unwrap();
    return my_int
}

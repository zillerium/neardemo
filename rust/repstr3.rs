fn repeat_str(src: &str, count: usize) -> String {
  std::iter::repeat(src).take(count).collect()
}

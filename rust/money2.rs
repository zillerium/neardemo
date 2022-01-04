fn bonus_time(salary: u64, bonus: bool) -> String {
  format!("¥{}", salary * if bonus {10} else {1})
}

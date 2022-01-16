fn jumping_number(n: u64) -> String {
    let is_jumping = format!("{}", n)
      .chars()
      .map(|x| x.to_digit(10).unwrap() as i32)
      .collect::<Vec<i32>>()
      .windows(2)
      .map(|xs| (xs[0] - xs[1]).abs() == 1)
      .fold(true, |acc, e| acc && e);
    return if is_jumping {
        String::from("Jumping!!")
    } else {
        String::from("Not!!")
    }
}

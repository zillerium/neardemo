use itertools::Itertools;
  
fn multi_table(n: u64) -> String {
    (1..=10)
        .map(|a| format!("{} * {} = {}", a, n, a * n))
        .join("\n")
}

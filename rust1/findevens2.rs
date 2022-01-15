fn even_numbers(xs: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut res: Vec<i32> = xs.iter().rev().cloned().filter(|x| x % 2 == 0).take(n).collect();
    res.reverse();
    res
}

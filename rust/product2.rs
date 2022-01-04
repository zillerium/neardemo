fn grow(array: Vec<i32>) -> i32 {
    array.iter().fold(1, |acc, x| acc * x)
}

fn first_non_consecutive(arr: &[i32]) -> Option<i32> {
    arr.windows(2).find(|s| s[0] + 1 != s[1]).map(|s| s[1])
}

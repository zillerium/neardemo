fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
  for i in 1..arr.len() {
    if arr[i] - arr[i - 1] != 1 {
      return Some(arr[i]);
    }
  };
  None
}

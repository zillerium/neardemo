fn find_smallest_int(arr: &[i32]) -> i32 {
    // your code here
    let mut min = arr[0];
    for i in arr {
        if i < &min {
            min = *i;
        }    
    }
    return min;
}

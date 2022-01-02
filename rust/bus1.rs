fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    if on + wait > cap {
        return wait - cap + on;
    } else {
        return 0;
    }
}

fn index(nums: &[u64], n: usize) -> Option<u64> {
    match nums.get(n) {
        Some(i) => { return Some(i.pow(n as u32)); }
        None => { return None; }
    }
}

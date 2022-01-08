fn index(nums: &[u64], n: usize) -> Option<u64> {
   // todo!();
    if nums.len() > n {
    Some (nums[n].pow(n as u32))
    } else {
        None
    }    
}

fn find_average(slice: &[f64]) -> f64 {
   if slice.is_empty() { return 0.0 as f64};
       let sum: f64 =  slice.iter().sum();
    let len = slice.len() as  f64;
    sum/len
}

fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
    // Good luck!
     let y:Vec<i32> = array.iter().copied().filter(|s| s % 2 == 0).collect();
   println!("{}",y.len());
    y[y.len()-number..y.len()].to_vec()
 
}

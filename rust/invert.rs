fn invert(values: &[i32]) -> Vec<i32> {
     values.iter().map(|s| s*(-1)).collect()

    
}
fn main() {
 
    println!("{:?}",  invert(&vec![1,2,3,4,5]));
   
 
}

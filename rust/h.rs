fn next_id(ids: &[usize]) -> i32 {
 println!("{:?}", ids);
    let mut max = ids[0];
    let mut min = ids[0];
    for i in ids {
        if *i > max {
            max = *i;
        }
    } 
    
   for i in ids {
        if *i < min {
            min = *i;
        }
    }

   println!("{:?} {:?}", min,max);
   
   let mut a :Vec<usize> = Vec::new();
   for i in ids {
       a.push(*i);
   }
   let mut v = a;
   println!("{:?}", v.sort());
   
   return 0;
 
}
fn main() {
    println!("{:?}",next_id(&[0,1,5,7,4,5]));
    
}

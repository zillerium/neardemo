fn getevens(myseries: &Vec<i32>) -> Vec<i32> {
   //  let even:Vec<i32> =myseries.retain(|x| *x%2 == 0);
     myseries.iter().copied().filter(|a, b| a -b ).collect()

    
}

fn main() { 
    let myseries = vec![1,2,3,4,5,5];
    println!("{:?}",getevens(&myseries));
}


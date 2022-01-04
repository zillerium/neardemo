fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
  // code here
    let mut last_i = arr[0];
    let mut noncon = arr[0];
    let mut count = 0;
    while count < arr.len() && noncon == arr[0] {
        let  i = arr[count];
        if i == last_i + 1 {
            // consecutive
            last_i = i;
        } else {
            noncon = i;
        }
        count +=1;
    }
    println!("{:?} {:?}", last_i, arr[0]);
    if last_i == arr[arr.len()-1] { return None; } else { return Some(noncon);}
     
 
}

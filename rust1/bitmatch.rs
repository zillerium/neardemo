fn add(x: i32, y: i32) -> i32 {
  //  unimplemented!()
    let mut x1 = x;  
    let mut y1 = y;
    while y1!=0 {
        let carry = x1 & y1;
        println!("{}", carry);
        x1 = x1 ^ y1;
        y1 = carry << 1;
 
    }
    x1
}

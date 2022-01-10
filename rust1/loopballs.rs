fn pyramid(balls: u16) -> u16 {
    //todo!()
    let mut tot= balls;
    //let mut count = 0;
    let mut i = 0;
   // (0..=balls).fold(0, |sum, item| sum + item)
    // 0,1,2,3,4
    while tot > i {
        i+=1;
        tot-=i;
    }
 /*   for i in 0..balls {
        println!("{} coun", i);
        if tot>i {
            tot-=i;
            count +=1;
        } else {
           // break;
        }
   */     
  //  }
    i
    

}

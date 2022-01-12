fn count_salutes(hallway: &str) -> usize {
    // todo!()
     println!("{:?}", hallway);
     let mut right = Vec::new();
        let mut left = Vec::new();
     let mut _g = 0;
     for i in 0..hallway.len() {
         if hallway.chars().nth(i).unwrap() == '>' {
             right.push(i);      
         }
      }
      for i in 0..hallway.len() {
        if hallway.chars().nth(i).unwrap() == '<' {
            left.push(i);      
        }
     }
   /*   for (_i,c) in hallway.chars().enumerate() {
        match ">".find(c) {
           Some(_i)=>right.push(_i),
           None=>_g+=1
        }
       };
       for (_i,c) in hallway.chars().enumerate() {
         match "<".find(c) {
           Some(_i)=>left.push(_i),
           None=>_g+=1
         }
       };
       */
       let mut count = 0;
     println!("{:?}", left);
       println!("{:?}", right);
       for i in &right {
             for j in &left {
                 if *j > *i {
                     count+=1;
                 }
             }   
       } ;   
        count * 2     
   //    ">" 5 ,10
   //       "<" 1, 6, 7
   //       i) count number > 5 count1
   //       ii) count number  > 10 count2
   //  }  
     
 }

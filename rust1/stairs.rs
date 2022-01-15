fn elevator_distance(floors: &[i16]) -> i16 {
   // todo!()
    let mut tot = 0;
    for i in 0..floors.len()-1 {
       
            tot += (floors[i+1]- floors[i]).abs();
            
        
    }
    tot
    
}

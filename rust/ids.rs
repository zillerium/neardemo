fn next_id(ids: &[usize]) -> usize {
 println!("{:?}", ids);
    
    let mut prev: usize = ids[0];
    let mut minv: usize = ids[0];
    
    for i in ids.sort() {
        if *i > prev + 1 {
            if prev + 1 < minv
            { minv =  prev + 1 }        
        } else {
            prev = *i;
        }
    }
    if minv == ids[0] { return prev + 1;} else {
        return minv;
    }
 
}

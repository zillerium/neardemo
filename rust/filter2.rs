 
fn main() {
    
    
     println!("{}",  count_sheep(&[true, false, "", true]))
}

fn count_sheep(sheep: &[bool]) -> u8 {
    let mut count = 0;
    for &i in sheep {
        
        if i {
            count=count+1;
        }
    }
    return count;
}

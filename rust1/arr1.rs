fn main() {
    let n = 145;
    let char_vec: Vec<char> = n.to_string().chars().collect();
         let mut a: Vec<char> = Vec::new();
  

     let len_n:u32 = char_vec.len() as u32;
        for i in 0..len_n {
            let y = char_vec[i as usize] as char;
            a.push(y);        
        }
     
        let _start: String = a.iter().collect();
}

fn test (n:u32)-> bool {
    let g = format!("{:b}", n);
    let bin_v: Vec<char> = g.to_string().chars().collect();
    let l = bin_v.len()-1;
    println!("{}", bin_v[0]);
    println!("{}", bin_v[l]);
    if bin_v[0] == '1' && bin_v[l] == '1' {
        true
    } else {
        false
    }
}


fn main() { 
    let n = 3; //0,1,2,3
 // len = 4, len/2 = 2, m-1, m
   let mut a: Vec<u32> = Vec::new();
    for i in 0..n {
        if test(i) {
            a.push(i);
        }
        
    }
    println!("{:?} ", a  );
         
    
}


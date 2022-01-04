 

 
fn main() {

    //348597 => [7,9,5,8,4,3]
   let x = "12567".to_string().chars().rev().collect::<String>();
     
    println!("{:?}", x.chars());
    let mut v: Vec<u8> = Vec::new();
    for   i in x.chars() {
        println!("{:?}", i);
        let h = (i.to_string()).parse::<u8>().unwrap();
         println!("{:?}", h);
        v.push(h);
    }
    println!("{:?}", v);
    }

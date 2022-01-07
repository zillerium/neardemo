fn strings() {

        let s:&'static str = "hello there!";
        for c in s.chars() {
            println!("{}", c);
        }


        if let Some(fchar) = s.chars().nth(0) {
           println!("fchar {}", fchar);
        }

        let mut letters = String::new();
        let mut a = 'a' as u8;
        while a <= ('z' as u8) {
            letters.push(a as char);
            letters.push_str(",");
            a +=1;
        }
        println!("{}", letters);
        let u:&str = &letters;
        let z = letters + "abc";
}

fn main() {
    strings();
    let vec = vec![3,2,1];

//    for x in vec.iter_mut() {
  //      println!("{}", x);
   //     *x+=8;
  //  }

    for x in vec.iter().rev() {
        println!("{}", x);
    }

    for x in &vec {
        println!("{:?}", *x);
    }

    let mut vec2 = vec![3,4,5];
//    let it = vec2.into_iter();
    vec2.extend(vec);
    println!("{:?}",vec2);
}

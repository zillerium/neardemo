#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;
struct Point {

    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{ x: 0.0, y: 0.0}
}

pub fn stack_and_heap() {

   let p1 = origin();
   let p2 = Box::new(origin());

    println!("{:?}", mem::size_of_val(&p1));
    println!("{:?}", mem::size_of_val(&p2));
let temp = 89;
let day = if temp > 20 {"sunny"} else {"cold"};
println!("{:?}", day);
let mut x = 1;
while x < 1000 {
x*=2;
if x == 64 { continue;};

    println!("{}", x);

}


}


fn main() {

    stack_and_heap();

//    println!("{:?}", x);
}

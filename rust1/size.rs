#![allow(dead_code)]
#![allow(unused_variables)]

const PI: u8 = 38;
static Z: i32 = 122;

use std::mem;

fn main() {
    let a = 222;
    println!("{:?}", mem::size_of_val(&a));
    println!("{:?}", Z);

    let d: char = 'x';

    println!("{:?}", mem::size_of_val(&d));

    let acubed = i32::pow(a, 3);

    println!("{:?}", acubed);

    let b = 4.7;
    // let b = 34.7;
    let bcubed1 = f64::powi(b, 3);
    let bcubed2 = f64::powf(b, 3.0);

    println!("{:?}", bcubed1);
    println!("{:?}", bcubed2);
    {
        let x = 2 | 2;

        println!("{:?}", x);
        let t = 1 << 10; // shift 10 places to the right, hence 2 pow 10
        println!("{:?}", t);
    }

    //    println!("{:?}", x);
}

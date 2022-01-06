#[derive(Clone,Copy)]

union IntOrFloat
{
i:i32,
f:f32
}

fn process_value(iof: IntOrFloat) {

    unsafe {
        match iof {
           IntOrFloat { i: 42 } => {
               println!("value 42");
           }
           IntOrFloat{ f } => {println!("none");}
        }
    }
}

fn main() {
let mut iof = IntOrFloat{ i:42 };
process_value(iof);
unsafe{
    println!("{}", iof.i);
}
}

fn is_divide_by(number: i32, a: i32, b: i32) -> bool {
    if number % a == 0 && number % b==0 
    { return true;} else {return false;}
}

fn main() {
   println!("{}",is_divide_by(10000, 5, -3))
}

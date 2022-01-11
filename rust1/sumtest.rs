fn main() {
    let a = 10;
    let b = a;
    println!("{}",sum(a,b));

}

fn sum(a: u32, b: u32) -> u32 {
    a+b
}

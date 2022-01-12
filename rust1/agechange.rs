fn main() {
let a=90;
changeage(a);
println!("{}", a);
}

fn changeage(mut a: u32) -> u32 {
    a+=9;
    a
}

fn main() {
    let x = 3.0;
    let y = 20.0;

    let res = if y != 0.0 { Some(x / y) } else { None };

    match res {
        Some(z) => println! {"yes"},
        None => println! {"no"},
    }

    if let Some(z) = res {
        println!("result {}", z)
    }
}


fn main() {

    let message = String::from("hello");
    let message = extend_message(message);
    println!("{}", message);

}

fn extend_message(mut a:String) -> String {

    a.push_str(" world");
    a
}

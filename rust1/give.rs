fn take(vec: Vec<i32>) {

}

fn main() {

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    take(vec);
    vec.push(4); // gives an error as fn call moves vec
}

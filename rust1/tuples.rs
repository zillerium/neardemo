fn sum_and_product(x:i32, y:i32) -> (i32, i32) {
(x+y, x*y)
}

fn tuples() {

    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);
    let (a,b) = sp;
    println!("{:?}", a)
}

fn main() {

tuples();

}

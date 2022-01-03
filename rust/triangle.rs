fn triangle(n: i32) -> i32 {
    let mut sum = 0;
    for i in 0..n+1 {
        sum += i;
    }
    sum
}
fn main() {
 
    println!("{:?}",  triangle(1));
       println!("{:?}",  triangle(2));
          println!("{:?}",  triangle(3));
   println!("{:?}",  triangle(4));
 
}

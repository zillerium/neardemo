fn main() {//   let mut v = [1, 2, 3, 4, 5];
 
 //println!("{:?}",v.reverse() );
 let n = 10;
  println!("{}", (1..n).fold(0, | sum, item| sum + item));
 

}

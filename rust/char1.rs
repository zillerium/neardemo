fn main() {

   println!("{:?}", match char::from_u32(97) { Some(c) => c, None => '$' });
}

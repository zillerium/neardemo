fn how_many(x:i32) -> &'static str {
match x {
        0 => "no",
        1 | 2 => "one or two",
        9...12 => "more",
        _ if (x % 2 == 0) => "even some",
        
        _ => "few"
}

}

fn pattern_matching() {
   for x in 0..13 {
      println!("{} {}", x, how_many(x));
   }
}

struct Point<T> {
    x: T,
    y: T
}

fn generics() {

    let a:Point<i32> = Point{ x:0, y:0};
    println!("{}", a.x);
}

fn main() {

    pattern_matching();
    generics();
}

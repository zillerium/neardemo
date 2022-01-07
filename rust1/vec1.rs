
fn vectors() -> Vec<i32> {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(11);
    a
}

fn p1(x:i32) {
println!("{} printline 1", x);
}


fn p(x:i32) {
println!("{} printline ", x);
}

fn main() {
    let id:usize = 0;
    let mut a = vectors();
    println!("{:?}",vectors()[id]);
    match a.get(2) {
        Some(x) => println!("found {}", x),
        None => println!("error")
    
    }
    let y = a.pop();
    for i in &a {
        //p(*i);
        p(match y{Some(y)=>y, None=>0});
    }

    while let Some(x) = a.pop() {
       p1(x);
    }

        while let Some(x) = a.pop() {
       p1(x);
    }



}

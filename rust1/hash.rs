use std::collections::HashMap;

fn main() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.entry("circle".into()).or_insert(1);
    println!("{}", shapes["triangle".into()]);

    {
        let a = shapes.entry("circle".into()).or_insert(1);
        *a = 0;
    }

    for (k, v) in shapes {
        println!("{} {}", k, v);
    }
}

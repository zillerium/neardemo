struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    Cmyk { cyan: u8, magenta: u8, yellow: u8 },
}

fn enums() {
    //let c:Color = Color::RgbColor(0,0,0);
    let c: Color = Color::Cmyk {
        cyan: 0,
        magenta: 128,
        yellow: 0,
    };
    match c {
        Color::Red => println!("r"),
        Color::Blue => println!("b"),
        Color::Green => println!("g"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb{},{},{}", r, g, b),
        Color::Cmyk {
            cyan: 0,
            magenta: 128,
            yellow: _,
        } => println!("magneta"),
        Color::Cmyk {
            cyan: _,
            magenta: _,
            yellow: _,
        } => println!("magneta not"),
    }
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 31.0, y: 41.0 };
    println!("{} {}", p.x, p.y);
    let myline = Line { start: p, end: p2 };

    println!("{} {}", myline.start.x, myline.end.x);
}

pub fn main() {
    enums();
    structures();
}

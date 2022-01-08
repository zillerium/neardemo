use rand::Rng;
use std::io::stdin;

fn main() {

    let number = rand::thread_rng().gen_range(1,101);
    loop {
        println!("enter name ");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
          Ok(_) => {
            let parsed = buffer.trim_end().parse::<i64>();
            match parsed {
              Ok(guess) => {
                println!("ok");
                break;
              }
              Err(e) => {
                println!("err {}", e);
              }
            }
          },
          Err(_) => continue,

        }
    }
}

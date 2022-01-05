#![allow(dead_code)]
#![allow(unused_variables)]

//use rand::Rng;
use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked
}

fn main() {


let code = String::from("1234");

let mut state = State::Locked;
let mut entry = String::new();

loop {

    match state {
        State::Locked => {
            let mut input = String::new();
            match stdin().read_line(&mut input) {
                Ok(_) => {
                    entry.push_str(&input.trim_end());
                }
                Err(_) => continue
            }

            if entry == code {
                state = State::Unlocked;
                continue;
            }

            if !code.starts_with(&entry) {
                state = State::Failed;
            }
        } 
        State::Failed => {
            println!("failed");
            entry.clear();
            state = State::Locked;
            continue;

        }
        State::Unlocked => {
            println!("unlocked");
            return;
        }
    }
}
/*
let country_code = 44;
let country = match country_code {
    44 => "UK",
    46 => "Sweden",
    1..=1000 => "unknown",
    _ => "error"
};




    println!("{:?}", country);
*/
}

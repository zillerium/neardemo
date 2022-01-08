use std::f32::consts::PI;

fn count_to_5() -> i32 {
    let mut foo1 = 0;
    loop {
        if foo1 > PI as i32 && foo1 > 5 {
           break;
        }
        foo1 += 1;
    }
    5
}
fn main() {
    println!("I can count to {}", count_to_5());
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
}

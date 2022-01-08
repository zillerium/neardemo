use std::collections::HashSet;
use std::thread;
use std::time;

fn main() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("gamma2");
    greeks.remove("gamma2");
    if greeks.insert("gamm88a") {
        println!("{:?}", greeks);
    }

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    println!(
        "is {:?} a subset of {:?}? {}",
        _2_8,
        _1_10,
        _2_8.is_subset(&_1_10)
    );
    println!(
        "is {:?} a subset of {:?}? {}",
        _2_8,
        _1_10,
        _2_8.is_disjoint(&_1_10)
    );
    println!(
        "is {:?} a subset of {:?}? {:?}",
        _2_8,
        _1_10,
        _2_8.union(&_1_10)
    );
}

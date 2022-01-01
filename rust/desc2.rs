fn descending_order(x: u64) -> u64 {
    if x == 0 { return 0};
       let mut x1 = x;
    let mut result = std::iter::from_fn(move || {
        if x1 == 0 {
            None
        } else {
            let current = x1 % 10;
            x1 /= 10;
            Some(current)
        }
    })
    .collect::<Vec<u64>>();

    result.reverse();

//let mut vec = vec![1,4,5,1,4];
result.sort_by(|a, b| b.cmp(a));
println!("{:?}", &result);
    let mut tot: u64 = 0;
    let mut counter: u32 = result.len() as u32;
    let base: i32= 10;
    for i in &result {
        let t: u64= base.pow(counter-1) as u64;
        tot += i * t;
        counter=counter -1;
    }
return tot;
}


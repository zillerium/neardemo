fn bin_to_decimal(inp: &str) -> i32 {
    let dec = isize::from_str_radix(inp, 2).unwrap() as i32;
    dec
}

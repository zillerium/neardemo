

fn switch_it_up(n: usize) -> &'static str {
    let x: i32 = n as i32;
    let word = match x  {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        _ => "none"
    };
    return word;
   
}

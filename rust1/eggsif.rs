fn cooking_time(eggs: u32) -> u32 {
    //todo!()
    if eggs % 8 == 0 {
        (eggs/8)*5
    } else {
        ((eggs/8)+1)*5
    }
}

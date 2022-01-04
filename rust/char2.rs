fn get_char(c: i32) -> char {
    //todo!()
    match char::from_u32(c as u32) { Some(c) => c, None => '$' }
}


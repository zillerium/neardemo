fn cooking_time(eggs: u32) -> u32 {
    ((eggs as f32 / 8.0).ceil()) as u32 * 5
}

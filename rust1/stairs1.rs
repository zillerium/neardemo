fn elevator_distance(floors: &[i16]) -> i16 {
    floors.windows(2).map(|s| (s[0] - s[1]).abs()).sum()
}

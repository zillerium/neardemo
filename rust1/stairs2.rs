use itertools::Itertools;

fn elevator_distance(floors: &[i16]) -> i16 {
    floors
        .iter()
        .tuple_windows()
        .map(|(origin, destination)| (origin - destination).abs())
        .sum()
}

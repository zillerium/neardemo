fn pyramid(balls: u16) -> u16 {
    ((1. + 8.*balls as f64).sqrt() as u16 - 1) / 2
}

fn count_sheep(sheep: &[bool]) -> u8 {
  sheep.iter().filter(|&&x|x).count() as u8
}

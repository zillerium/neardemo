fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
 if gallons * mpg < distance_to_pump { return false} else {return true}
}

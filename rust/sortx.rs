 
fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    
    if dir == 'L' {
     //   return cubes.sort_by_key(|a| Reverse(*a));
          return cubes.sort();
    } else {
        return cubes.sort();
    }
}

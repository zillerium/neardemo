fn get_count(string: &str) -> usize {
  let mut vowels_count: usize = 0;

  for (i,c) in string.chars().enumerate() {
      vowels_count = match "aeiou".find(c) {
          Some(i)=>vowels_count+1,
          None=>vowels_count
      }
      
    }  
    
  // Do your magic here
  
  vowels_count
}


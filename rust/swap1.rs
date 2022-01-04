fn name_shuffler(s: &str) -> String {
    //todo!()
   
      let test: Vec<&str> = s.split(' ').rev().collect();
    
      test.join(" ")

}

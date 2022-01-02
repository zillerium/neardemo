 
  
  fn update_light(current: &str) -> String {
  
  // green, to yellow, to red, and then to green again.
    // Code goes here.
    let lights = vec!["green", "yellow", "red"];
    if lights.contains(&current) {
        let index = lights.iter().position(|&r| r == current).unwrap();
        println!("{} index ", index);
        if index  == lights.len() - 1 {
            return lights[0].to_string();
        } else {
            return lights[index+1].to_string();
        }
   
    }
    return "green".to_string();
}  
    
 
fn main() {
 
   println!("{:?}", update_light("yellow"));
     println!("{:?}", update_light("red"));
     println!("{:?}", update_light("green"));
}

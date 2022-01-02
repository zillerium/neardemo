fn usdcny(usd: u16) -> String {
    let conv: f64 = 6.75;
     let   yx: f64 = usd as f64;
    let cny:f64 = yx  * conv;
    
   // let mut y = (cny * 100.0).round() / 100.0;
   // let y1 = (cny * 10.0).round() / 10.0;
        let curr = format!("{:.2}", cny);
  
 
    
    curr + " Chinese Yuan"
    
}
fn main() {
 
   println!("{:?}", usdcny(19750))
}

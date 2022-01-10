fn combat(health: f32, damage: f32) -> f32 {
    //unimplemented!();
    if health - damage < 0.0 {
        0.0
    } else {
        health - damage
    }
     
}

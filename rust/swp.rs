fn get_drink_by_profession(param: &str) -> &'static str {
    // code me!-
    
    let n = param.to_lowercase();
   
   
    
    match n.as_ref() {
        "jabroni" => return "Patron Tequila",
        "school counselor" => return "Anything with Alcohol",
       "programmer" => return "Hipster Craft Beer",
       "bike gang member" => return "Moonshine",
       "politician" => return "Your tax dollars",
       "rapper" => return "Cristal",
       _ => return "Beer"
    }
}    

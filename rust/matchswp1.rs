fn get_drink_by_profession(param: &str) -> &'static str {
    let lower_str: &str = &param.to_lowercase();
    
    match lower_str {
        "jabroni" => "Patron Tequila",
        "school counselor" => "Anything with Alcohol",
        "programmer" => "Hipster Craft Beer",
        "bike gang member" => "Moonshine",
        "politician" => "Your tax dollars",
        "rapper" => "Cristal",
        _ => "Beer"
    }
}

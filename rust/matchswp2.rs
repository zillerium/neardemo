fn get_drink_by_profession(prof: &str) -> &'static str {
    match prof.to_lowercase().as_str() {
        "jabroni"          => "Patron Tequila",
        "school counselor" => "Anything with Alcohol",
        "programmer"       => "Hipster Craft Beer",
        "bike gang member" => "Moonshine",
        "politician"       => "Your tax dollars",
        "rapper"           => "Cristal",
        _                  => "Beer"
    }
}

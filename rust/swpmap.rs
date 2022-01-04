use std::collections::HashMap;
fn get_drink_by_profession(param: &str) -> &'static str {
    let mut map: HashMap<String, &str> = HashMap::new();
    map.insert("jabroni".into(), "Patron Tequila");
    map.insert("programmer".into(), "Hipster Craft Beer");
    map.insert("school counselor".into(), "Anything with Alcohol");
    map.insert("bike gang member".into(), "Moonshine");
    map.insert("politician".into(), "Your tax dollars");
    map.insert("rapper".into(), "Cristal");


    match map.get(&param.to_lowercase()) {
        Some(item) => item,
        _ => "Beer"
    }
}

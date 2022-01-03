fn hello(name: &str) -> String {
    if name.is_empty() {
        return String::from("Hello, World!");
    }
    return format!(
        "Hello, {}{}!",
        name[..1].to_uppercase(),
        name[1..].to_lowercase()
    );
}

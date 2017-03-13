pub fn hello(possible_name: Option<&str>) -> String {
    format!("Hello, {}!", possible_name.unwrap_or("World"))
}

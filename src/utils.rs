use std::fs;

pub fn load_file(day: &str) -> String {
    let file = format!("data/{}.txt", day);
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
}

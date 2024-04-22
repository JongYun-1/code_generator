use std::fs;

#[derive(Debug)]
pub struct Config {
    pub search_pattern: String,
    pub base_search_intensity: u32,
    pub obstacle_handling: String,
}

impl Config {
    pub fn new(file_path: &str) -> Self {
        let contents = fs::read_to_string(file_path).expect("Error reading config file");
        let lines: Vec<&str> = contents.lines().collect();
        Config {
            search_pattern: lines[0].to_string(),
            base_search_intensity: lines[1].parse().unwrap(),
            obstacle_handling: lines[2].to_string(),
        }
    }
}

use std::fs;

pub fn get_ascii_logo() -> String {
    fs::read_to_string("img/logo2.txt")
        .expect("Failed to read logo.txt")
        .trim()
        .to_string()
}

pub fn get_ascii_sim() -> String {
    fs::read_to_string("img/simulation.txt")
        .expect("Failed to read simulation.txt")
        .trim()
        .to_string()
}
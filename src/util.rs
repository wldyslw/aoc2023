use std::fs;
use std::path::Path;

pub fn read_input(path: &str) -> String {
    match fs::read_to_string(Path::new(path)) {
        Ok(s) => s,
        Err(e) => {
            println!("Problems reading file: {e}");
            String::from("")
        }
    }
}

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct MyData {
    // Define your struct fields based on your JSON structure
    // Example:
    name: String,
    age: u32,
}

fn main() {
    // Open the file
    let mut file = match File::open("data.json") {
        Ok(file) => file,
        Err(_) => {
            println!("Error opening the file.");
            return;
        }
    };

    // Read the file contents into a string
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        println!("Error reading file contents.");
        return;
    }

    // Parse the JSON into your struct
    let parsed: Result<MyData, _> = serde_json::from_str(&contents);
    match parsed {
        Ok(data) => {
            println!("Parsed data: {:?}", data);
            // Now you can work with your data here
        }
        Err(err) => {
            println!("Error parsing JSON: {:?}", err);
        }
    }
}

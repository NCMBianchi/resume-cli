use colored::Colorize;
use serde_json::Value;
use std::fs;

pub fn show_contact() {
    let file_path = "./data/contact/contact.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let contact_data: Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    println!("");
    println!("You can contact me at:");
    println!(
        "Email: {}",
        contact_data["email"].as_str().unwrap().bright_red()
    );
    println!(
        "LinkedIn: {}",
        contact_data["linkedin"].as_str().unwrap().bright_blue()
    );
    println!(
        "Github: {}",
        contact_data["github"].as_str().unwrap().bright_purple()
    );
    println!(
        "Gitlab: {}",
        contact_data["gitlab"].as_str().unwrap().bright_yellow()
    );
    println!("");
}

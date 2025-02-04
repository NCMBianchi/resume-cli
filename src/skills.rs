use colored::Colorize;
use serde_json::{Result, Value};

pub fn show_skills(json_data: &str) -> Result<()> {
    let v: Value = serde_json::from_str(json_data)?;

    println!("");
    println!(
        "{}: {}",
        "Languages".bold(),
        v["Languages"].as_str().unwrap().bright_green()
    );
    println!(
        "{}: {}",
        "Web Technologies".bold(),
        v["Web Technologies"].as_str().unwrap().bright_green()
    );
    println!(
        "{}: {}",
        "AI/ML".bold(),
        v["AI/ML"].as_str().unwrap().bright_green()
    );
    println!(
        "{}: {}",
        "DevOps".bold(),
        v["DevOps"].as_str().unwrap().bright_green()
    );
    println!(
        "{}: {}",
        "Bioinformatics CLIs".bold(),
        v["Bioinformatics CLIs"].as_str().unwrap().bright_green()
    );
    println!(
        "{}: {}",
        "Visualization".bold(),
        v["Visualization"].as_str().unwrap().bright_green()
    );
    println!(
        "{}: {}",
        "Runtimes".bold(),
        v["Runtimes"].as_str().unwrap().bright_green()
    );
    println!(
        "{}: {}",
        "OS".bold(),
        v["OS"].as_str().unwrap().bright_green()
    );
    Ok(())
}

use colored::Colorize;
use serde_json::{Result, Value};

pub fn show_education(json_data: &str) -> Result<()> {
    let v: Value = serde_json::from_str(json_data)?;
    let mut i = 0;
    loop {
        let exp = &v["Education"][i];
        if exp.is_null() {
            break;
        }
        println!("");
        println!("{} #{}", "Education".bold(), (i + 1).to_string().bold());
        println!(
            "{} : {}",
            "Degree".red().bold(),
            exp["Degree"].as_str().unwrap().bright_red()
        );
        println!(
            "{} : {}",
            "University".yellow().bold(),
            exp["University"].as_str().unwrap().bright_green()
        );
        println!(
            "{} : {}",
            "Location".yellow().bold(),
            exp["Location"].as_str().unwrap().bright_green()
        );
        println!(
            "{} : {}",
            "Duration".yellow().bold(),
            exp["Duration"].as_str().unwrap().bright_green()
        );
        println!("{} :", "Description".yellow().bold());
        let mut j = 0;
        loop {
            let desc = &exp["Description"][j];
            if desc.is_null() {
                break;
            }
            println!("- {}", desc.as_str().unwrap().bright_green());
            j += 1;
        }
        i += 1;
    }

    Ok(())
}

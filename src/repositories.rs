use colored::Colorize;
use serde_json::{Result, Value};

pub fn show_repositories(json_data: &str) -> Result<()> {
    let v: Value = serde_json::from_str(json_data)?;
    let mut i = 0;
    loop {
        let exp = &v["Repositories"][i];
        if exp.is_null() {
            break;
        }
        println!("");
        println!(
            "{} #{}",
            "Main repositories".bold(),
            (i + 1).to_string().bold()
        );
        println!(
            "{} : {}",
            "Name".yellow().bold(),
            exp["Name"].as_str().unwrap().bright_green()
        );
        println!(
            "{} : {}",
            "Github Link".yellow().bold(),
            exp["github_link"].as_str().unwrap().bright_purple()
        );
        println!(
            "{} : {}",
            "Gitlab Link".yellow().bold(),
            exp["gitlab_link"].as_str().unwrap().bright_yellow()
        );
        println!(
            "{} : {}",
            "Languages".yellow().bold(),
            exp["Languages"].as_str().unwrap().bright_green()
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

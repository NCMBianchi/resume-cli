mod contact;
mod education;
mod experience;
mod repositories;
mod skills;

use colored::Colorize;
use contact::show_contact;
use education::show_education;
use experience::show_experience;
use inquire::Select;
use repositories::show_repositories;
use serde_json::Value;
use skills::show_skills;
use std::fs;

fn main() {
    let file_path = "./data/main/main.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let main_data: Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");
    let name = main_data["name"].as_str().unwrap();
    let intro = main_data["intro"].as_str().unwrap();
    let career_1 = main_data["career_1"].as_str().unwrap();
    let career_2 = main_data["career_2"].as_str().unwrap();
    let coding_skills = main_data["coding_skills"].as_str().unwrap();
    let general_skills = main_data["general_skills"].as_str().unwrap();

    println!("");
    println!("");
    println!(
        "Hey there! I'm {}, {}.",
        name.bold().bright_yellow(),
        intro.bold().bright_yellow()
    );

    let options = vec![
        "About Me",
        "Experience",
        "Education",
        "Skills",
        "Repositories",
        "Contact",
        "Exit",
    ];

    loop {
        let choice = Select::new("What would you like to know?", options.clone()).prompt();

        match choice {
            Ok(choice) => {
                if choice == options[0] {
                    println!("");
                    println!(
                        "I am a proficient and driven {} {}.",
                        career_1.bold().bright_yellow(),
                        career_2.bold().bright_yellow()
                    );
                    println!("I possess diverse technical skills, including proficiency in programming languages such as {}.",coding_skills.bold().bright_yellow());
                    println!("With practical knowledge in {} I have the capacity to tackle complex projects in scientific research via computational sciences.",general_skills.bold().bright_yellow());
                    println!("");
                } else if choice == options[1] {
                    let file_path = "./data/experience/experience.json".to_owned();
                    let contents =
                        fs::read_to_string(file_path).expect("Couldn't find or load that file.");
                    let res = show_experience(&contents);
                    match res {
                        Ok(_res) => println!(""),
                        Err(_) => println!("Error in experience.rs"),
                    }
                } else if choice == options[2] {
                    let file_path = "./data/education/education.json".to_owned();
                    let contents =
                        fs::read_to_string(file_path).expect("Couldn't find or load that file.");
                    let res = show_education(&contents);
                    match res {
                        Ok(_res) => println!(""),
                        Err(_) => println!("Error in education.rs"),
                    }
                } else if choice == options[3] {
                    let file_path = "./data/skills/skills.json".to_owned();
                    let contents =
                        fs::read_to_string(file_path).expect("Couldn't find or load that file.");
                    let res = show_skills(&contents);
                    match res {
                        Ok(_res) => println!(""),
                        Err(_) => println!("Error in skills.rs"),
                    }
                } else if choice == options[4] {
                    let file_path = "./data/repositories/repositories.json".to_owned();
                    let contents =
                        fs::read_to_string(file_path).expect("Couldn't find or load that file.");
                    let res = show_repositories(&contents);
                    match res {
                        Ok(_res) => println!(""),
                        Err(_) => println!("Error in repositories.rs"),
                    }
                } else if choice == options[5] {
                    show_contact();
                } else if choice == options[6] {
                    println!("Bye! Have a great day!");
                    break;
                }
            }
            Err(_) => println!("You did not select a valid option"),
        }
    }
}

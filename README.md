# Resume CLI
This is my adaptation of [karanpargal](https://github.com/karanpargal)'s command-line interface (CLI) portfolio made using Rust. It provides a simple and interactive way to showcase your internships/projects and skills in a terminal environment. With this portfolio, you can easily manage and present your work to others.

![CLI_test copia](https://github.com/user-attachments/assets/3106a981-1870-4990-b986-26cecf32c7a5)

## Features
- Interactive Command Line Interface: The portfolio is designed to be used in the terminal, providing an interactive and user-friendly experience.
- Experience Showcase: Add and display your projects with relevant details such as name, description, and GitHub repository link.
- Skills Showcase: Highlight your skills and expertise by adding them to your portfolio.
- Customizable: You can easily customize the portfolio by editing the data files to fit your own experiences and skills.

## Installation
To install and use the portfolio, you need to have Rust programming language installed on your system. Please follow the official Rust documentation to install [Rust](https://www.rust-lang.org/tools/install)

Once Rust is installed, you can clone this repository:
```
git clone https://github.com/NCMBianchi/resume-cli.git
```
Modify all the files storing the information you want to display (*i.e.* `contact.json`, `experience.json`, `skills.json` and `main.json` for the actual data; `contact.rs`, `experience.rs`, `skills.rs` and `main.rs` for styles). Then, you can build the project:
```
cargo build
```

After the build process is completed, you can run the portfolio:
```
cargo run
```

You can also eventually branch the [original project by karanpargal](https://github.com/karanpargal/resume-cli) –just like I also did.

## License
This project is licensed under the MIT License. Feel free to use and modify the code as per your needs.

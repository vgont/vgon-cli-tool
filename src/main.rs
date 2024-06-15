use clap::{command, Command};
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let matches = command!()
        .subcommand(Command::new("init").about("Initialize a new project"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        let options = &["Prisma", "typeorm", "Other"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your ORM")
            .default(0)
            .items(&options[..])
            .interact()
            .expect("Failed to make a selection");

        match selection {
            0 => println!("You chose prisma"),
            1 => println!("You chose typeorm"),
            2 => println!("You chose Other"),
            _ => println!("Invalid selection"),
        }
    }
}

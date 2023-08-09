use std::io;
use std::io::prelude::*;
use colored::*;
use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;
use crate::helpers::config::Config;
use crate::helpers::file::read_dir;

pub fn greet(config: &Config) {
    println!("color-swapper v0.1.0");
    println!("{}", "\nWelcome to color-swapper.".cyan());

    let items = read_dir(&config.input_folder).len();
    println!("Found {} items to swap.\n", format!("{}", items).yellow());

    if !Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap()
    {
        std::process::exit(0);
    }
}

pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "\nPress enter to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

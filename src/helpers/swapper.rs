use indicatif::ProgressBar;
use crate::helpers;
use crate::helpers::config::Config;
use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;
use colored::Colorize;

pub fn ask_swap() {
    if !Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap()
    {
        std::process::exit(0);
    }
}

pub fn swap(config: &Config) {
    let input_paths = helpers::file::read_dir(&config.input_folder);
    let progress_bar = ProgressBar::new(input_paths.len() as u64);

    for path in input_paths {
        let mut file_content = helpers::file::read_file(&path);

        for color in &config.colors {
            let from = &color.from;
            let to = &color.to;

            file_content = file_content.replace(from, to);
        }

        let output_path = path.replace(&config.input_folder, &config.output_folder);
        helpers::file::write_file(&output_path, &file_content);
        progress_bar.inc(1);
    }

    progress_bar.finish_and_clear();
    println!("{} {}\n", "Done!".green(), "🎉".cyan());
}
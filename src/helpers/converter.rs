use colored::Colorize;
use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;
use indicatif::ProgressBar;
use crate::helpers;
use crate::helpers::config::Config;

pub fn ask_convert() {
    if !Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Do you want to convert the files to {}?", ".png".yellow()))
        .interact()
        .unwrap()
    {
        std::process::exit(0);
    }
}

pub fn convert(config: &Config) {
    let output_paths = helpers::file::read_dir(&config.output_folder);
    let progress_bar = ProgressBar::new(output_paths.len() as u64);

    for path in output_paths {
        let output_path = path.replace(&config.input_folder, &config.output_folder);

        helpers::svg::convert(&output_path);
        helpers::file::delete_file(&output_path);
        progress_bar.inc(1);
    }

    progress_bar.finish_and_clear();
    println!("{} {}", "Done!".green(), "🎉".cyan());
}
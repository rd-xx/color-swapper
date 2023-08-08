mod helpers {
    pub mod config;
    pub mod file;
    pub mod program;
}

fn main() {
    let config = helpers::config::read_config();

    helpers::file::ensure_dir(&config.input_folder);
    helpers::file::ensure_dir(&config.output_folder);
    helpers::file::clear_dir(&config.output_folder);

    let input_paths = helpers::file::read_dir(&config.input_folder);
    for path in input_paths {
        let mut file_content = helpers::file::read_file(&path);

        for color in &config.colors {
            let from = &color.from;
            let to = &color.to;

            file_content = file_content.replace(from, to);
        }

        let output_path = path.replace(&config.input_folder, &config.output_folder);
        helpers::file::write_file(&output_path, &file_content);

        println!("File {} done.", path.split("/").last().unwrap());
    }

    helpers::program::pause();
}

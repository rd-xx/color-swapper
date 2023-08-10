mod helpers {
    pub mod config;
    pub mod converter;
    pub mod file;
    pub mod program;
    pub mod svg;
    pub mod swapper;
}

fn main() {
    let config = helpers::config::read_config();

    helpers::file::ensure_dir(&config.input_folder);
    helpers::file::ensure_dir(&config.output_folder);
    helpers::file::clear_dir(&config.output_folder);

    helpers::program::greet(&config);

    helpers::swapper::ask_swap();
    helpers::swapper::swap(&config);

    helpers::converter::ask_convert();
    helpers::converter::convert(&config);

    helpers::program::pause();
}

mod helpers {
    pub mod config;
    pub mod file;
}

fn main() {
    println!("Hello, world!");
    helpers::file::ensure_dir("test");

    let config = helpers::config::read_config();
    println!("config: {}", config.input_folder);
}

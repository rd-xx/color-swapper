use std::fs;

pub fn ensure_dir(path: &str) {
    println!("ensure_dir: {}", path);

    fs::create_dir_all(path).expect("Couldn't create the directory");
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Couldn't read the file")
}
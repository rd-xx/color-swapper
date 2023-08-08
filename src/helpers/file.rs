use std::fs;

pub fn ensure_dir(path: &str) {
    fs::create_dir_all(path).expect("Couldn't create the directory");
}

pub fn clear_dir(path: &str) {
    fs::remove_dir_all(path).expect("Couldn't clear the directory");
    ensure_dir(path);
}

pub fn read_dir(path: &str) -> Vec<String> {
    let entries= fs::read_dir(path).expect("Couldn't read the directory");
    let mut files: Vec<String> = Vec::new();

    for entry in entries {
        let entry = entry.expect("Couldn't read the entry");
        let meta = entry.metadata().expect("Couldn't read the metadata");

        if meta.is_file() {
            if let Some(v) = entry.path().to_str() {
                files.push(v.to_string());
            }
        }
    }

    files = files.into_iter().filter(|x| !x.contains(".DS_Store")).collect();

    return files;
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Couldn't read the file")
}

pub fn write_file(path: &str, content: &str) {
    fs::write(path, content).expect("Couldn't write the file");
}

pub fn delete_file(path: &str) {
    fs::remove_file(path).expect("Couldn't delete the file");
}
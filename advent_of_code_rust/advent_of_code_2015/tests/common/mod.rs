use std::fs::File;

pub fn input_file_exists(path: &str) -> bool {
    if let Err(e) = File::open(path) {
        eprintln!("Input file unreachable at {:?}: {}", path, e);
        return false
    }
    true
}
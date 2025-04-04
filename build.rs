use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::env;

fn main() {
    // Get the project root directory (where Cargo.toml is)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let problems_dir = format!("{}/src/Problems", manifest_dir);
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("includes.rs");
    let mut file = File::create(dest_path).unwrap();

    // Write the header
    writeln!(file, "// Auto-generated includes").unwrap();
    
    // Make the script more robust by handling the case when directory doesn't exist
    match fs::read_dir(&problems_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                        let path_str = path.to_str().unwrap();
                        // Convert backslashes to forward slashes
                        let normalized_path = path_str.replace('\\', "/");
                        writeln!(file, "include!(r\"{}\");", normalized_path).unwrap();
                    }
                }
            }
        },
        Err(_) => {
            // Write a comment if the directory doesn't exist
            writeln!(file, "// No problem files found in {}", problems_dir).unwrap();
        }
    }
}
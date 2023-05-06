use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use chrono::{Datelike, Utc};

fn main() -> std::io::Result<()> {
    let now = Utc::now();
    let dir_path = Path::new("journal-entries");
    if !dir_path.exists() {
        std::fs::create_dir(dir_path)
            .expect("Failed to create directory");
    }

    let mut entry = String::new();
    println!(
        "Journal Entry for {}",
        now.to_rfc2822()
    );
    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read entry");

    let file_path = dir_path.join("test-entry.txt");
    File::create(file_path)
        .expect("Failed to create file")
        .write_all(entry.as_bytes())
        .expect("Failed to write file");

    Ok(())
}

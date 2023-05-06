use std::fs::File;
use std::io;
use std::io::Write;

fn main() {
    println!("Journal Entry for <today's date>:");

    let mut entry = String::new();
    
    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read entry");

    File::create("entry.txt")
        .expect("Failed to create file")
        .write_all(entry.as_bytes())
        .expect("Failed to write file");
}

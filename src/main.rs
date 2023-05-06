use std::fs::File;
use std::io;
use std::io::Write;

fn main() {
    println!("Journal Entry for <today's date>:");

    let mut entry = String::new();
    
    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read entry");

    println!("Journal Entry: {}", entry);

    let mut file = File::create("entry.txt");
    file.expect("Failed to save entry").write_all(entry.as_bytes());
}

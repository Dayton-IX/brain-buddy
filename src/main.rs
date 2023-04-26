use std::io;

fn main() {
    println!("Journal Entry for <today's date>:");

    let mut entry = String::new();
    
    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read entry");

    println!("Journal Entry: {}", entry);
}

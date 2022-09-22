use std::io;

fn main() {
    println!("Enter a string to reverse:");

    let mut string_to_reverse = String::new();

    io::stdin()
        .read_line(&mut string_to_reverse)
        .expect("Failed to read line");

    let reversed_string: String = string_to_reverse.trim().chars().rev().collect::<String>();

    println!("Reverse, reverse: {reversed_string}");
}

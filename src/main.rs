use std::io;

fn main() {
    println!("Welcome to the temp converter, what do you need converting?");
    println!("Do you want to convert (1) Celsius to Fahrenheit or (2) Fahrenheit to Celsius?");
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");

    println!("You chose: {}", user_input.trim());

    match user_input.trim() {
        "1" => println!("Converting Celsius to Fahrenheit"),
        "2" => println!("Converting Fahrenheit to Celsius"),
        _ => println!("Invalid choice!"),
    }
}

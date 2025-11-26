use std::io;

fn main() {
    println!("Welcome to the temp converter, what do you need converting?");
    println!("Do you want to convert (1) Celsius to Fahrenheit or (2) Fahrenheit to Celsius?");
    let mut conversion_choice = String::new();

    io::stdin()
        .read_line(&mut conversion_choice)
        .expect("Failed to read user input");

    println!("You chose: {}", conversion_choice.trim());

    match conversion_choice.trim() {
        "1" => {
            println!("Converting Celsius to Fahrenheit");
            println!("Enter the temperature in Celcius you would like to convert:");
            let mut temperature_input = String::new();
            io::stdin()
                .read_line(&mut temperature_input)
                .expect("Failed to read temperature input");
            let mut temperature_to_convert: f64 = temperature_input
                .trim()
                .parse()
                .expect("Failed to convert temp string to float");
            let fahrenheit_result = (temperature_to_convert * 2.0) + 30.0;
            println!(
                "{} Degrees Celcius to Fahrenheit is {} Fahrenheit",
                temperature_input.trim(),
                fahrenheit_result
            );
        }
        "2" => println!("Converting Fahrenheit to Celsius"),
        _ => println!("Invalid choice!"),
    }
}

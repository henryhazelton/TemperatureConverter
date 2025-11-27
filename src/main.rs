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
            let temperature_to_convert: f64 = temperature_input
                .trim()
                .parse()
                .expect("Failed to convert temp string to float");
            let fahrenheit_result = (temperature_to_convert * 1.8) + 32.0;
            println!(
                "{} Degrees Celcius to Fahrenheit is {} Degrees Fahrenheit",
                temperature_input.trim(),
                fahrenheit_result
            );
        }
        "2" => {
            println!("Converting Fahrenheit to Celsius");
            println!("Enter the temperature in Fahrenheit you would like to convert:");
            let mut temperature_input = String::new();
            io::stdin()
                .read_line(&mut temperature_input)
                .expect("Failed to read temperature input");
            let temperature_to_convert: f64 = temperature_input
                .trim()
                .parse()
                .expect("Failed to convert temp string to float");
            let celcius_result = (temperature_to_convert - 32.0) / 1.8;
            println!(
                "{} Degrees Fahrenheit is {} Degrees Celcius",
                temperature_input.trim(),
                celcius_result
            );
        }
        _ => {
            println!("Invalid choice!")
        }
    }
}

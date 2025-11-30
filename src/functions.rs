use std::io;

pub fn get_temp_input() -> String {
    let mut temperature_input = String::new();
    io::stdin()
        .read_line(&mut temperature_input)
        .expect("Failed to read temperature input");
    temperature_input
}

pub fn convert_string_to_float(temp_input: &str) -> f64 {
    temp_input
        .trim()
        .parse::<f64>()
        .expect("Failed to convert temp string to float")
}

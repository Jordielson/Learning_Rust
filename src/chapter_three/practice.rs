use std::io;

// Convert temperatures between Fahrenheit and Celsius
fn convert_fahrenheit_to_celsius(fahrenheit_temperature: f32) -> f32 {
    (fahrenheit_temperature - 32.0) * (5.0/9.0)
}

fn convert_celsius_to_fahrenheit(celsius_temperature: f32) -> f32 {
    (celsius_temperature * (9.0/5.0)) + 32.0
}

pub fn convert_between_fahrenheit_celsius() -> () {
    let mut scale: String = String::new();
    let mut temperature: String = String::new();

    println!("Choose the temperature to be converted (C or F): ");
    io::stdin().read_line(&mut scale).expect("Failed to read line");
    println!("Temperature: ");
    io::stdin().read_line(&mut temperature).expect("Failed to read line");

    let temperature: f32 = temperature
        .trim()
        .parse()
        .expect("Temperature entered was not a number");

    let scale: String = scale.trim().to_uppercase();

    let converted_temperature: f32 = if scale.eq("F") {
        convert_fahrenheit_to_celsius(temperature)
    } else if scale.eq("C") {
        convert_celsius_to_fahrenheit(temperature)
    } else {
        panic!("Invalid temperature scale. Use 'C' to Celsius or 'F' to Fahrenheit.")
    };

    println!("The temperature converted is {}", converted_temperature)
}


// Generate the nth Fibonacci number
use num_bigint::BigUint;
use num_traits::One;

pub fn generate_fibonacci_number(number: u32) -> BigUint {
    let mut a = BigUint::one();
    let mut b = BigUint::one();

    for _ in 2..=number {
        let next = &a + &b;
        a = std::mem::replace(&mut b, next);
    }

    b
}
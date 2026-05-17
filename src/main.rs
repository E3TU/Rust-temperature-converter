use std::io;

fn celcius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celcius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    loop {
        println!("Which value you want to convert");
        println!("1. Celcius to fahrenheit");
        println!("2. Fahrenheit to celcius");

        let mut option = String::new();

        io::stdin().read_line(&mut option).expect("Failed to read line");

        let number: i32 = option.trim().parse().expect("Please enter a valid integer");

        println!("you entered, {number}");
    }
}

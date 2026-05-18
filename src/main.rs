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

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let number: i32 = option.trim().parse().expect("Please enter a valid integer");

        if number == 1 {
            println!("Enter temperature in celcius");
            let mut celcius = String::new();

            io::stdin()
                .read_line(&mut celcius)
                .expect("Failed to read line");

            let celcius: f64 = celcius.trim().parse().expect("Please enter a valid number");

            let fahrenheit: f64;

            fahrenheit = celcius_to_fahrenheit(celcius);

            println!("");
            println!("Temperature in fahrenheit: {fahrenheit}");
            println!("");
        } else if number == 2 {
            println!("Enter temperature in fahrenheit");
            let mut fahrenheit = String::new();

            io::stdin()
                .read_line(&mut fahrenheit)
                .expect("Failed to read line");

            let fahrenheit: f64 = fahrenheit.trim().parse().expect("Enter a valid number");

            let celcius: f64;

            celcius = fahrenheit_to_celcius(fahrenheit);

            println!("");
            println!("Temperature in celcius: {celcius}");
            println!("");
        }
    }
}

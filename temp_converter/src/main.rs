use std::io;

fn main() {
    loop {
        println!("Do you want to convert temperature from:(1) Fahrenheit to Celsius or (2) Celsius to Fahrenheit?");
        println!("(1) Fahrenheit to Celsius.");
        println!("(2) Celsius to Fahrenheit.");
        println!("Enter your input:(1/2)");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read choice");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please enter 1 or 2.");
                continue;
            }
        };

        match choice {
            1 => {
                let fahrenheit = get_temp_input("Fahrenheit");
                let celsius = fahrenheit_to_celsius(fahrenheit);
                println!("Temperature in Celsius is {:.2}", celsius);
            }
            2 => {
                let celsius = get_temp_input("Celsius");
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!("Temperature in Fahrenheit is {:.2}", fahrenheit);
            }
            _ => {
                println!("Invalid choice. Please enter 1 or 2.");
                continue;
            }
        }
        break;
    }
}

fn get_temp_input(scale: &str) -> f32 {
    let mut temp = String::new();
    println!("Enter temperature in {}", scale);
    io::stdin().read_line(&mut temp).expect("Failed to read temperature");
    let temp: f32 = temp.trim().parse().expect("Failed to parse temperature");
    temp
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}

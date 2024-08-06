use std::{io};

fn main() {
    let mut a = String::new();
    println!("Enter value of a:");
    io::stdin().read_line(&mut a).expect("Failed to read A");
    let a: f32 = a.trim().parse().expect("Please enter a valid number for a");

    let mut oprtn = String::new();
    println!("Enter operator:('+','-','*','/')");
    io::stdin()
        .read_line(&mut oprtn)
        .expect("Failed to read operator");

    let mut b = String::new();
    println!("Enter value of b:");
    io::stdin().read_line(&mut b).expect("Failed to read B");
    let b: f32 = b.trim().parse().expect("Please enter a valid number for b");

    match oprtn.trim() {
        "+" => println!("{} + {} = {}", a, b, a + b),
        "-" => println!("{} - {} = {}", a, b, a - b),
        "*" => println!("{} * {} = {}", a, b, a * b),
        "/" => {
            if b != 0.0 {
                println!("{} / {} = {}", a, b, a / b)
            } else {
                println!("Division by zero is not allowed")
            }
        }
        _ => println!("Invalid operator"),
    }
}

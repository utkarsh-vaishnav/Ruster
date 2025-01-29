use std::{thread, time::Duration};
use std::io;

fn main() {
    println!("Enter countdown time (in seconds) :");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let seconds: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! Please enter a number.");
            return;
        }
    };

    for i in (1..=seconds).rev() {
        println!("{}", i);
        thread::sleep(Duration::from_secs(1));
    }

    println!("⏰ Time's up!");
}

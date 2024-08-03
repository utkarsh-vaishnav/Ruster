use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("GUESSING THE NUMBER GAME...!");
    println!("-----------------------------------------------------");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Enter the player name:");
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name).expect("Failed to read player_name");
    let player_name = player_name.trim();
    loop {
        println!("Please your random guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations...Guessed number is  correct.");
                println!("You win {}!",player_name);
                break;
            }
        }
    }
}
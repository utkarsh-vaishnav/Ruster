use rand::seq::SliceRandom;
use std::io;

fn main() {
    println!("Welcome to Hangman game...!");
    println!("This game is for car lovers. Random word is a famous car brand...");
    println!("Let's see your CAR LOVE...");
    let words = vec![
        "lamborghini",
        "bugatti",
        "ferrari",
    ];
    let mut rng = rand::thread_rng();
    let word = words.choose(&mut rng).expect("Failed to select word");

    let mut guessed_letters = vec!['_'; word.len()];
    let mut attempts = 7;

    loop {
        println!("\nWord: {}", guessed_letters.iter().collect::<String>());
        println!("Attempts left: {}", attempts);

        println!("Please guess a letter:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess = input.trim().chars().next().expect("No input");

        let mut found = false;
        for (i, letter) in word.chars().enumerate() {
            if letter == guess {
                guessed_letters[i] = letter;
                found = true;
            }
        }

        if !found {
            attempts -= 1;
            println!("Incorrect guess!");
        }

        if guessed_letters.iter().all(|&c| c != '_') {
            println!("Congratulations! You guessed the word: {}", word);
            break;
        }

        if attempts == 0 {
            println!("Sorry, you've run out of attempts. The word was: {}", word);
            break;
        }
    }
}

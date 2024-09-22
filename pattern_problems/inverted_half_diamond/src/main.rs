use std::io;

fn main() {
    println!("Enter the number of rows: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().parse::<i32>().unwrap();

    for i in 0..input {
        for _ in 0..input-i-1 {
            print!(" ");
        }
        for _ in 0..i+1 {
            print!("*");
        }
        println!();
    }
    
    for i in 1..input {
        for _ in 0..i {
            print!(" ");
        }
        for _ in 0..input-i {
            print!("*");
        }
        println!();
    }
}

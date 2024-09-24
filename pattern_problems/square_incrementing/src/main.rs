use std::io;

fn main() {
    println!("Square Incrementing Pattern");
    println!("Enter the number of rows:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().parse::<i32>().unwrap();

    for i in 1..=input{
        for _j in 1..=input{
            print!("{i}");
        }
    println!();
    }
}

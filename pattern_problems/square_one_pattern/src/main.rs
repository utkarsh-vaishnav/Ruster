use std::io;

fn main() {
    println!("Enter the number of rows:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().parse::<i32>().unwrap();
    println!("Your Pattern:");
    for _each in 1..=input{
        for _each in 1..=input{
            print!("1 ");
        }
    println!();
    }
}

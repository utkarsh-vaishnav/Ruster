use std::io;
fn main() {
    println!("Right Triangle Number Pattern");
    println!("Enter number of lines:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().parse::<i32>().unwrap();
    let mut num = 1;

    for i in 1..=input{
        for _ in 1..=i{
            print!("{}",num);
            num+=1;
        }
        println!();
    }
}

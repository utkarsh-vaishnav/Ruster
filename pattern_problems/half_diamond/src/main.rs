fn main() {
    println!("Enter the number of rows: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().parse::<i32>().unwrap();

    for i in 1..=input {
        for _ in 0..i {
            print!("* ");
        }
        println!();
    }

    for i in (1..=input).rev() {
        for _ in 0..i {
            print!("* ");
        }
        println!();
    }



}

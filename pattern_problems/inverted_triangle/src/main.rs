fn main() {
    println!("Enter the number of rows: ");
    let mut rows = String::new();
    std::io::stdin().read_line(&mut rows).expect("Failed to read input");
    let rows = rows.trim().parse::<i32>().unwrap();
    
    for i in (1..=rows).rev() {
        for _ in 0..i {
            print!("* ");
        }
        println!();
    }
}

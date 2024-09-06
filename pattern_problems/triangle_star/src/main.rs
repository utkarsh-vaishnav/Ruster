fn main() {
    println!("Enter the number of rows: ");
    let mut rows = String::new();
    std::io::stdin().read_line(&mut rows).unwrap();
    let rows: u32 = rows.trim().parse().unwrap();

    for i in 1..=rows {
        for _ in 0..i {
            print!("*");
        }
        println!();
    }
}

fn main() {
    println!("Enter the number of rows: ");
    let mut rows = String::new();
    std::io::stdin().read_line(&mut rows).unwrap();
    let rows = rows.trim().parse::<i32>().unwrap();

    for i in 1..=rows {
        for _ in 1..=rows -i {
            print!(" ");
        }
        for _ in 0..2 * i - 1 {
            print!("* ");
        }
        println!();
    }
}

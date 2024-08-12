use std::io;

fn leap_year(year: u32){
    if (year % 4 == 0) && (year % 100 != 0) || (year % 400 == 0) {
        println!("{} is a leap year", year);
    } else {
        println!("{} is not a leap year", year);
    }
}

fn main() {
    let mut year = String::new();
    println!("Enter the year:");
    io::stdin().read_line(&mut year).expect("Failed to read line");
    let year:u32 = year.trim().parse().expect("Please type a number!");
    leap_year(year);
}

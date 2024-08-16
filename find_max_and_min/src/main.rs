use std::io;

fn main() {
    println!("Enter values of array(space between 2 values): ");
    let mut array=String::new();
    io::stdin().read_line(&mut array).expect("Failed to read line");

    let arrayn: Vec<u32> = array
    .trim()
    .split_whitespace()
    .map(|s| s.parse().expect("parse error"))
    .collect();

    let mut max:u32 = arrayn[0];
    let mut min:u32 = arrayn[0];

    for i in 1..arrayn.len() {
        if arrayn[i] > max {
            max = arrayn[i];
        }
        if arrayn[i] < min {
            min = arrayn[i];
        }
    }
    // let max = arrayn.iter().max().unwrap();
    // let min = arrayn.iter().min().unwrap();
    println!("Max: {}, Min: {}", max, min);
}

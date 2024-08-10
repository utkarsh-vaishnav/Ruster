use std::io;

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let mut data = String::new();
    println!("Enter word or string you want: ");
    io::stdin().read_line(&mut data).expect("Failed to read line");
    let data = data.trim();
    let reverse_data = reverse_string(data);
    if data == reverse_data{
        println!("Data is palindrome");
    } else {
        println!("Data is not palindrome");
    }
    println!("String: {data} & Reversed String: {reverse_data}");
}

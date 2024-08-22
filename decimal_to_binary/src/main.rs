fn decimal_to_binary(mut n: u32) -> String {
    let mut binary_num = String::new();
    while n > 0 {
        let remainder = n % 2;
        binary_num = format!("{}{}", remainder, binary_num);
        n = n / 2;
    }
    binary_num
}

fn main() {
    println!("Enter Number: "); 
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let number = input.trim().parse::<u32>().unwrap();
    let binary_representation = decimal_to_binary(number);
    println!("Binary of decimal_number:{} is {}", number, binary_representation);
}

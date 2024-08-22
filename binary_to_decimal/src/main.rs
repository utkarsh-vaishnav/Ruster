fn binary_to_decimal(binary_str: &str) -> u32 {
    let mut decimal_num = 0;
    let mut power = binary_str.len() as u32 - 1;

    for digit in binary_str.chars() {
        let digit_value = digit.to_digit(10).unwrap();
        decimal_num += digit_value * (2u32.pow(power));
        if power > 0 {
            power -= 1;
        }
    }
    decimal_num
}

fn main() {
    println!("Enter Binary Number: "); 
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let number = input.trim().parse::<u32>().unwrap();
    let decimal_representation = binary_to_decimal(number.to_string().as_str());
    println!("Decimal of binary_number:{} is {}", number, decimal_representation);
}

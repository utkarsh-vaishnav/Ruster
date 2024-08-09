use std::io;

fn factorial(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let mut n = String::new();
    println!("Enter the value of n:");
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n = n.trim().parse::<i32>().unwrap();

    let factorial_value = factorial(n);
    println!("Factorial of {} is {}", n, factorial_value);

}

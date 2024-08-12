use std::io;

fn fibonacci(n: u32)-> u32 {
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            // println!("{} {}", fibonacci(n - 1), fibonacci(n - 2));
            fibonacci(n - 1) + fibonacci(n - 2)
        }
    }

    fn main() {
        let mut n = String::new();
        println!("Enter Value of n:");
        io::stdin().read_line(&mut n).unwrap();
        let n: u32 = n.trim().parse().unwrap();
    
        for i in 0..=n {
            println!("{}", fibonacci(i));
        }
    }
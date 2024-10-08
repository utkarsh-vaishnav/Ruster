use std::io;

// // Recursive function
// fn fibonacci(n: u32)-> u32 {
//         if n == 0 {
//             0
//         } else if n == 1 {
//             1
//         } else {
//             // println!("{} {}", fibonacci(n - 1), fibonacci(n - 2));
//             fibonacci(n - 1) + fibonacci(n - 2)
//         }
//     }

// Iterative function
// fn fibonacci(n: u32) -> u32 {
//     if n == 0 {
//         return 0;
//     } else if n == 1 {
//         return 1;
//     }

//     let mut a = 0;
//     let mut b = 1;
//     let mut c = 0;

//     for _ in 2..=n {
//         c = a + b;
//         a = b;
//         b = c;
//     }
//     // println!("{} {} {}", a, b, c);
//     c
// }

// Dynamic programming
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut fib = vec![0; (n + 1) as usize];
    fib[1] = 1;

    for i in 2..=n as usize {
        fib[i] = fib[i - 1] + fib[i - 2];
    }
    fib[n as usize]
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

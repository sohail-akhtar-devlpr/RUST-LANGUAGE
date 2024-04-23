// Q-4

use std::io;

// Function to check if a given number is prime
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false; 
    }

    // Check divisibility from 2 to the square root of n
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    // Prompt the user to enter a number
    println!("Enter a number:");

    // Read user input from the standard input stream (stdin)
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Convert the user input to an unsigned 64-bit integer
    let num: u64 = input.trim().parse().unwrap();

    // Check if the number is prime
    if is_prime(num) {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }
}

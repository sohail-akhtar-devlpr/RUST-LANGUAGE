// Q-10

use std::io;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let num: u64 = input.trim().parse().unwrap();

    // Check if the number is prime
    let is_prime = if num <= 1 {
        false
    } else {
        let mut i = 2;
        while i * i <= num {
            if num % i == 0 {
                return println!("{} is not prime", num);
            }
            i += 1;
        }
        true
    };

    if is_prime {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }
}

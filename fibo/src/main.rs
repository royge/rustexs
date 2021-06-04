use std::io;
use fibo::fibonacci;

fn main() {
    println!("Input a number:");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to line");

    let n = n.trim().parse().expect("Please type a number");
    let n = fibonacci(n);

    println!("The fibonacci of your number is {}.", n)
}

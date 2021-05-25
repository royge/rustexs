use std::io;
use tempconvert::convert;

fn main() {
    println!("Input temperature in Fahrenheit:");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to line");

    let temp = temp.trim().parse().expect("Please type a number");
    let temp = convert(temp);

    println!("Temperature in Celsius: {}", temp)
}

use std::cmp::PartialOrd;

fn main() {
    let numbers = vec![12, 34, 13, 78, 23];
    let result = largest(&numbers);
    println!("Largest number: {}", result);

    let chars = vec!['d', '2', 'b', 'x'];
    let result = largest(&chars);
    println!("Largest character: {}", result);

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 }; 

    println!("x: {}", both_integer.x());
    println!("y: {}", both_float.y());
    println!("x: {}, y: {}", integer_and_float.x(), integer_and_float.y());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

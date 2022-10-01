use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

fn main() {
    let pair = Pair::new(12, 34);
    pair.cmp_display();

    let pair = Pair::new(50.56, 34.34);
    pair.cmp_display();

    let pair = Pair::new("G", "A");
    pair.cmp_display();

    println!("Pair: {}", pair.to_string())
}

pub struct Pair<T> {
    pub y: T,
    pub x: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}

impl<T: Display> Display for Pair<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

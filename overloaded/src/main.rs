use std::fmt::{Display, Formatter, Result};
use std::ops::Add;

fn main() {
    assert_eq!(
        Point { x: 4, y: 100 } + Point { x: 50, y: 20 },
        Point { x: 54, y: 120 },
    );

    assert_eq!(Millimeters(250) + Meters(2), Millimeters(2250),);

    let person = Human;
    assert_eq!(String::from("dreaming"), person.fly(),);
    assert_eq!(String::from("flying an airplane"), Pilot::fly(&person),);
    assert_eq!(String::from("flying a broom"), Wizard::fly(&person),);

    assert_eq!(String::from("Coco"), Dog::baby_name());
    assert_eq!(String::from("puppy"), <Dog as Animal>::baby_name());

    let point = Point { x: 10, y: 5 };
    assert_eq!(String::from("** (10, 5) **"), point.star_string());

    assert_eq!(
        Wrapper(vec![String::from("Hello"), String::from("Rust!")]).to_string(),
        String::from("Hello, Rust!"),
    )
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);

struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("flying an airplane")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("flying a broom")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("dreaming")
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Coco")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait StarString: Display {
    fn star_string(&self) -> String {
        let output = self.to_string();
        String::from(format!("** {} **", output))
    }
}

impl StarString for Point {}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0.join(", "))
    }
}

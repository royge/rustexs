use std;

fn main() {
    let fav_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "20".parse();

    let mut results = String::new();

    if let Some(_color) = fav_color {
        results.push_str("got my favorite color");
    } else if is_tuesday {
        results.push_str("today is tuesday");
    } else if let Ok(age) = age {
        if age > 30 {
            results.push_str("age is greather than 30");
        } else {
            results.push_str("age is less than 30");
        }
    } else {
        results.push_str("no match");
    }

    assert_eq!("age is less than 30", results);

    let v = vec!["a", "b", "c"];
    results.clear();

    for (i, v) in v.iter().enumerate() {
        results.push_str(format!("{}=>{} ", i, v).as_str());
    }
    assert_eq!("0=>a 1=>b 2=>c ", results);

    results.clear();

    let (a, b, c) = (1, 2, 3);
    results.push_str(format!("{}, {}, {}", a, b, c).as_str());
    assert_eq!("1, 2, 3", results);

    let p = &(12, 34);
    let results = format_coordinates(p);
    assert_eq!("coordinates: 12, 34", results);

    // let Some(_) = fav_color;
    // if let x = 5 {
    //     println!("5");
    // }

    let x = 1;

    let mut results = "";
    match x {
        1 => results = "one",
        2 => results = "two",
        3 => results = "three",
        _ => results = "any",
    }
    assert_eq!("one", results);

    let x = Some(5);
    let y = 10;

    let mut results = String::new();
    match x {
        Some(50) => results.push_str("got 50"),
        Some(y) => results = format!("matched y = {}", y),
        _ => (),
    }
    assert_eq!("matched y = 5", results);
    assert_eq!(10, y);

    let k = 1;
    let mut results = "";
    match k {
        1 | 2 => results = "one or two",
        3 => results = "three",
        _ => results = "any",
    }
    assert_eq!("one or two", results);

    match k {
        1..=5 => results = "one to five",
        _ => results = "any",
    }
    assert_eq!("one to five", results);

    let c = 'a';

    match c {
        'a'..='j' => results = "early chars",
        'k'..='q' => results = "late chars",
        _ => results = "any",
    }
    assert_eq!("early chars", results);

    let p = Point { x: 1, y: 2 };

    let Point { x: a, y: b } = p;
    assert_eq!(1, a);
    assert_eq!(2, b);

    assert_ne!(std::ptr::addr_of!(p.x), std::ptr::addr_of!(a));

    let q = &p;
    assert_eq!(1, q.x);
    assert_eq!(2, q.y);

    assert_eq!(std::ptr::addr_of!(p.x), std::ptr::addr_of!(q.x));

    let Point { x, y } = p;
    assert_eq!(1, x);
    assert_eq!(2, y);

    assert_ne!(std::ptr::addr_of!(x), std::ptr::addr_of!(a));

    let p = Point { x: 0, y: 2 };
    let mut results = String::new();
    match p {
        Point { x, y: 0 } => results = format!("on x axis at {}", x),
        Point { x: 0, y } => results = format!("on y axis at {}", y),
        Point { x, y } => results = format!("neither axis: ({}, {})", x, y),
    }
    assert_eq!("on y axis at 2", results);

    let msg = Message::ChangeColor(0, 132, 250);
    results.clear();
    match msg {
        Message::Quit => {
            results.push_str("quit!");
        }
        Message::Move { x, y } => {
            results = format!("move to x: {}, y: {}", x, y);
        }
        Message::Write(text) => {
            results = format!("write string: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            results = format!("change color to RGB: {}, {}, {}", r, g, b);
        }
        _ => (),
    }
    assert_eq!("change color to RGB: 0, 132, 250", results);

    let msg = Message::ChangeNamedColor(Color::Hsv(2, 100, 200));
    results.clear();
    match msg {
        Message::ChangeNamedColor(Color::Rgb(r, g, b)) => {
            results = format!("change color to RGB: {}, {}, {}", r, g, b);
        }
        Message::ChangeNamedColor(Color::Hsv(h, s, v)) => {
            results = format!("change color to HSV: {}, {}, {}", h, s, v);
        }
        _ => (),
    }
    assert_eq!("change color to HSV: 2, 100, 200", results);

    let ((feet, inches), Point { x, y }) = ((6, 4), Point { x: 5, y: -10 });
    assert_eq!(6, feet);
    assert_eq!(4, inches);
    assert_eq!(5, x);
    assert_eq!(-10, y);

    let mut value = Some(4);
    let new_value = Some(20);
    let mut results = "";
    match (value, new_value) {
        (Some(_), Some(_)) => {
            results = "can't be";
        }
        _ => {
            value = new_value;
        }
    }
    assert_eq!("can't be", results);
    assert_eq!(Some(4), value);

    let nums = (10, 23, 56, 17, 45);
    match nums {
        (first, _, third, _, fifth) => {
            assert_eq!(10, first);
            assert_eq!(56, third);
            assert_eq!(45, fifth);
        }
    }

    let s = Some(String::from("Rust"));
    let mut results = "";
    if let Some(_s) = s {
        results = "found some string";
    }
    assert_eq!("found some string", results);
    // assert_eq!(Some(String::from("Rust")), s);

    let s = Some(String::from("Rust"));
    let mut results = "";
    if let Some(_) = s {
        results = "found some string";
    }
    assert_eq!("found some string", results);
    assert_eq!(Some(String::from("Rust")), s);

    let origin = SpatialPoint {
        x: 20,
        y: 67,
        z: 100,
    };

    let mut results = String::new();
    match origin {
        SpatialPoint { x, .. } => results = format!("x is {}", x),
    }
    assert_eq!("x is 20", results);

    let nums = (1, 2, 3, 4, 5);
    match nums {
        (first, .., last) => {
            results = format!("first: {}, last: {}", first, last);
        }
    }
    assert_eq!("first: 1, last: 5", results);

    let num = Some(67);
    match num {
        Some(x) if x < 100 => results = format!("less than 100: {}", x),
        Some(x) => results = format!("x: {}", x),
        None => (),
    }
    assert_eq!("less than 100: 67", results);

    let x = Some(10);
    let y = 5;

    match x {
        Some(50) => results.push_str("got 50"),
        Some(n) if n == y => results = format!("matched, n: {}", n),
        _ => results = format!("default, x: {:?}", x),
    }
    assert_eq!("default, x: Some(10)", results);

    results.clear();
    let z = true;
    match y {
        3 | 4 | 5 if z => results.push_str("ok!"),
        _ => (),
    }
    assert_eq!("ok!", results);

    let g = Greetings::Hello { id: 32 };
    match g {
        Greetings::Hello {id : id_var @ 30..=50} => {
            results = format!("found id in range: {}", id_var);
        },
        Greetings::Hello {id : 60..=80} => {
            results.push_str("found in other range");
        },
        Greetings::Hello {id} => {
            results = format!("found other id: {}", id)
        }
    }
    assert_eq!("found id in range: 32", results);
}

fn format_coordinates(&(x, y): &(i32, i32)) -> String {
    return format!("coordinates: {}, {}", x, y);
}

struct Point {
    x: i32,
    y: i32,
}

struct SpatialPoint {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ChangeNamedColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Greetings {
    Hello { id: i32 },
}

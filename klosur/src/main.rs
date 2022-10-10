fn main() {}

#[test]
fn borrowed() {
    let x = 5;

    let equal_to_x = |z| z == x;

    let y = 5;

    assert!(equal_to_x(y));

    println!("x: {}", x);
}

#[test]
fn moved() {
    let x = vec![2, 5, 8];

    let equal_to_x = move |z| z == x;

    let y = vec![2, 5, 8];

    assert!(equal_to_x(y));

    // println!("can't print x: {:?}", x);
}

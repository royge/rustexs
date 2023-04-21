fn main() {
    assert_eq!(20, add_twice(add_one, 9));
}

fn add_one(arg: i32) -> i32 {
    arg + 1
}

fn add_twice(f: fn(arg: i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

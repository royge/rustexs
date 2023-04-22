use makru::my_vec;
use std::vec::Vec;

fn main() {
    let v1 = my_vec![1, 2, 3];

    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    assert_eq!(v1, v2);
}

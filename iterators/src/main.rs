fn main() {}

#[test]
fn iterator_demo() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    // NOTE: can't access v1_iter anymore because it is owned by sum().
    // println!("{:?}", v1_iter.next());

    assert_eq!(total, 6);
}

#[test]
fn produce_other_iterators() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    name: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|shoe| shoe.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 8,
            name: String::from("sneaker"),
        },
        Shoe {
            size: 9,
            name: String::from("sandal"),
        },
        Shoe {
            size: 8,
            name: String::from("boot"),
        },
    ];

    let my_shoes = shoes_in_my_size(shoes, 8);

    assert_eq!(
        my_shoes,
        vec![
            Shoe {
                size: 8,
                name: String::from("sneaker")
            },
            Shoe {
                size: 8,
                name: String::from("boot")
            },
        ],
    );
}

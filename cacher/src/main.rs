use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    generate_workout(20, 20);
    generate_workout(30, 3);
}

fn generate_workout(intensity: u32, random_num: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut expensive_result = UIntCacher::new(expensive_closure);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Today, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_num == 3 {
            println!("Take a break!")
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity))
        }
    }
}

struct UIntCacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> UIntCacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> UIntCacher<T> {
        UIntCacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, args: u32) -> u32 {
        match self.values.get(&args) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(args);
                self.values.insert(v, v);
                v
            }
        }
    }
}

struct Cacher<'a, T, U, V>
where
    T: Fn(&U) -> V,
    U: Hash + Eq + ?Sized,
    V: Copy,
{
    calculation: T,
    values: HashMap<&'a U, V>,
}

impl<'a, T, U, V> Cacher<'a, T, U, V>
where
    T: Fn(&U) -> V,
    U: Hash + Eq + ?Sized,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<'a, T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, args: &'a U) -> V {
        match self.values.get(args) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(args);
                self.values.insert(args, v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_u32_values() {
    let mut c = UIntCacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);
    let v10 = c.value(10);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(v10, 10);
}

#[test]
fn call_with_different_integer_values() {
    let mut c = Cacher::new(|&a| a);

    let v1 = c.value(&1);
    let v2 = c.value(&2);
    let v10 = c.value(&10);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(v10, 10);
}

#[test]
fn call_with_different_string_values() {
    let mut c = Cacher::new(|a: &String| a.len());
    let str1 = &String::from("hello");
    let str2 = &String::from("world");
    let str3 = &String::from("hi");

    let v1 = c.value(str1);
    let v2 = c.value(str2);
    let v10 = c.value(str3);

    let w1: usize = 5;
    let w2: usize = 5;
    let w3: usize = 2;

    assert_eq!(v1, w1);
    assert_eq!(v2, w2);
    assert_eq!(v10, w3);
}

#[test]
fn call_with_different_vector_values() {
    let mut c = Cacher::new(|a: &[String]| a.len());

    let str1 = &[String::from("hello")];
    let str2 = &[
        String::from("world"),
        String::from("world"),
        String::from("world"),
        String::from("world"),
    ];
    let str3 = &[
        String::from("hi"),
        String::from("hi"),
        String::from("hi"),
    ];

    let v1 = c.value(str1);
    let v2 = c.value(str2);
    let v10 = c.value(str3);

    let w1: usize = 1;
    let w2: usize = 4;
    let w3: usize = 3;

    assert_eq!(v1, w1);
    assert_eq!(v2, w2);
    assert_eq!(v10, w3);
}

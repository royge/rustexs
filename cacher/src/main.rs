use std::thread;
use std::time::Duration;
use std::collections::HashMap;

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

    let mut expensive_result = Cacher::new(expensive_closure);

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

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
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

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);
    let v10 = c.value(10);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(v10, 10);
}

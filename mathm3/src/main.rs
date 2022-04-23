fn main() {
    println!("Hello, world!");
}

fn mean(list:Vec<i32>) -> f64 {
    let mut total:i32 = 0;
    let mut count: i32 = 0;
    for v in &list {
        total += *v;
        count += 1;
    }

    f64::from(total) / f64::from(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(mean(v), 4.5);

        let v = vec![9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24, 25];
        assert_eq!(mean(v), 16.75);
    }
}

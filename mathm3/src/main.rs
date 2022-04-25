fn main() {
    println!("Hello, world!");
}

fn mean(list: &Vec<i32>) -> f64 {
    let mut total: i32 = 0;

    // NOTE: count is being used because I struggled to convert usize to f64.
    let mut count: i32 = 0;
    for v in list {
        total += *v;
        count += 1;
    }

    f64::from(total) / f64::from(count)
}

fn median(mut list: Vec<i32>) -> f64 {
    // Unstable sort is fine for me because I prefer speed.
    list.sort_unstable();

    let len = list.len();
    let mid = len / 2;

    let mut median = f64::from(list[mid]);

    if len % 2 == 0 {
        median = f64::from(&list[mid - 1] + &list[mid]) / 2.0;
    }

    median
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![1, 2, 4, 7, 5, 6, 8, 3];
        assert_eq!(mean(&v), 4.5);
        assert_eq!(median(v), 4.5);

        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(mean(&v), 5.0);
        assert_eq!(median(v), 5.0);

        let v = vec![
            9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24, 25,
        ];
        assert_eq!(mean(&v), 16.75);
        assert_eq!(median(v), 15.5);
    }
}

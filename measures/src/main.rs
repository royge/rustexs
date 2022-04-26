use std::collections::HashMap;

fn main() {
    println!("Measures of central tendency!");
}

struct CentralTendency {
    inputs: Vec<i32>,
}

impl CentralTendency {
    pub fn new(mut inputs: Vec<i32>) -> Self {
        inputs.sort_unstable();
        Self { inputs }
    }

    pub fn mean(&self) -> f64 {
        mean(&self.inputs)
    }

    pub fn median(&self) -> f64 {
        median(&self.inputs)
    }

    pub fn mode(&self) -> Vec<i32> {
        mode(&self.inputs)
    }
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

fn median(list: &Vec<i32>) -> f64 {
    let len = list.len();
    let mid = len / 2;

    let mut median = f64::from(list[mid]);

    if len % 2 == 0 {
        median = f64::from(&list[mid - 1] + &list[mid]) / 2.0;
    }

    median
}

fn mode(list: &Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for v in list {
        let count = map.entry(*v).or_insert(0);
        *count += 1
    }

    let mut mode: Vec<i32> = Vec::new();

    let mut max = 1;
    for (k, v) in &map {
        if *v == max {
            mode.push(*k);
        }
        if *v > max {
            max = *v;

            mode.clear();
            mode.push(*k);
        }
    }

    mode.shrink_to_fit();
    mode.sort_unstable();

    mode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![1, 2, 4, 7, 5, 6, 8, 3];
        v.sort_unstable();
        assert_eq!(mean(&v), 4.5);
        assert_eq!(median(&v), 4.5);

        let v = vec![1, 2, 4, 7, 5, 6, 8, 3];
        assert_eq!(mode(&v), vec![1, 2, 3, 4, 5, 6, 7, 8]);

        let v = vec![1, 2, 2, 3, 4, 7, 5, 6, 8, 3];
        assert_eq!(mode(&v), vec![2, 3]);

        let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        v.sort_unstable();
        assert_eq!(mean(&v), 5.0);
        assert_eq!(median(&v), 5.0);

        let mut v = vec![
            9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24, 25,
        ];
        v.sort_unstable();
        assert_eq!(mean(&v), 16.75);
        assert_eq!(median(&v), 15.5);

        let v = vec![
            9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24, 25,
        ];
        assert_eq!(mode(&v), vec![13]);

        let ct = CentralTendency::new(vec![
            9, 10, 12, 15, 15, 13, 13, 13, 16, 16, 18, 22, 23, 24, 24, 25,
        ]);
        assert_eq!(ct.mean(), 16.75);
        assert_eq!(ct.median(), 15.5);
        assert_eq!(ct.mode(), vec![13]);
    }
}

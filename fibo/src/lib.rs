pub fn fibonacci(n:i32) -> i32 {
    let mut v = 0;

    if n == 0 {
        return v;
    }

    if n <= 2 {
       return 1;
    }

    v += fibonacci(n-1) + fibonacci(n-2);

    return v;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut n = 0;
        let mut fib = 0;
        assert_eq!(fib, fibonacci(n));

        n = 1;
        fib = 1;
        assert_eq!(fib, fibonacci(n));

        n = 2;
        fib = 1;
        assert_eq!(fib, fibonacci(n));

        n = 3;
        fib = 2;
        assert_eq!(fib, fibonacci(n));

        n = 4;
        fib = 3;
        assert_eq!(fib, fibonacci(n));

        n = 5;
        fib = 5;
        assert_eq!(fib, fibonacci(n));

        n = 6;
        fib = 8;
        assert_eq!(fib, fibonacci(n));

        n = 20;
        fib = 6765;
        assert_eq!(fib, fibonacci(n));
    }
}

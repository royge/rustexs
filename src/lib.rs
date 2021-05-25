pub fn convert(fah:f32) -> f32 {
    (fah - 32.00) / 1.8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let fah = 100.00;
        let celsius = 37.77778;
        assert_eq!(celsius, convert(fah));
    }

    #[test]
    fn it_should_not_works() {
        let fah = 100.00;
        let celsius = 38.77778;
        assert_ne!(celsius, convert(fah));
    }
}

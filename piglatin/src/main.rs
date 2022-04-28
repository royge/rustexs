use std::io;

fn main() {
    println!("Lets speak in pig latin!");
    println!("Type your word:");

    let mut s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to line");

    let res = piglatin(String::from(s.trim()));

    println!("In pig latin, it is: {:?}", res);
}

fn piglatin(word: String) -> String {
    let vowels = "aeiou";
    let mut result = String::new();
    let mut first_letter = String::new();
    let mut is_vowel = false;
    for c in word.chars() {
        if c.is_alphabetic() && first_letter == "" {
            first_letter = c.to_string();
            is_vowel = vowels.contains(c);
            if is_vowel {
                result.push(c);
            } else {
                continue;
            }
        } else {
            result.push(c);
        }
    }
    if is_vowel {
        result.push_str("hay")
    } else {
        result.push_str(first_letter.as_str());
        result.push_str("ay");
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = String::from("table");
        assert_eq!(piglatin(word), "abletay");

        let word = String::from("agent");
        assert_eq!(piglatin(word), "agenthay");

        let word = String::from("wonderful");
        assert_eq!(piglatin(word), "onderfulway");

        let word = String::from("koalā");
        assert_eq!(piglatin(word), "oalākay");

        let word = String::from("atē");
        assert_eq!(piglatin(word), "atēhay");

        let word = String::from("atē");
        assert_eq!(piglatin(word), "atēhay");

        let word = String::from("百家姓");
        assert_eq!(piglatin(word), "家姓百ay");
    }
}

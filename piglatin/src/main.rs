use std::io;

fn main() {
    println!("Lets speak in pig latin!");
    println!("Type your word:");

    let mut s = String::new();

    io::stdin().read_line(&mut s).expect("Failed to line");

    let res = piglatin(String::from(s.trim()));

    println!("In pig latin, it is: {:?}", res);
}

fn piglatin(word: String) -> String {
    let vowels = "aeiou";
    let mut result = word;
    let mut first_letter = String::new();
    for c in result.chars() {
        first_letter = c.to_string();
        break;
    }

    let s = first_letter.as_str();

    let is_vowel = vowels.contains(s) || vowels.to_uppercase().contains(s);

    if is_vowel {
        result.push_str("hay")
    } else {
        let len = first_letter.len();

        first_letter.push_str("ay");
        result.replace_range(..len, "");

        result.push_str(first_letter.as_str());
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

        let word = String::from("Agent");
        assert_eq!(piglatin(word), "Agenthay");

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

        let word = String::from("تقويم");
        assert_eq!(piglatin(word), "قويمتay");
    }
}

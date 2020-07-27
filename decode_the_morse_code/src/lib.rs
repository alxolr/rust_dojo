use std::collections::HashMap;
extern crate regex;
use regex::Regex;

pub struct MorseDecoder {
    morse_code: HashMap<String, String>,
}

impl MorseDecoder {
    pub fn new() -> Self {
        let morse_code = vec![
            (".-", "A"),
            ("-...", "B"),
            ("-.-.", "C"),
            ("-..", "D"),
            (".", "E"),
            ("..-.", "F"),
            ("--.", "G"),
            ("....", "H"),
            ("..", "I"),
            (".---", "J"),
            ("-.-", "K"),
            (".-..", "L"),
            ("--", "M"),
            ("-.", "N"),
            ("---", "O"),
            (".--.", "P"),
            ("--.-", "Q"),
            (".-.", "R"),
            ("...", "S"),
            ("-", "T"),
            ("..-", "U"),
            ("...-", "V"),
            (".--", "W"),
            ("-..-", "X"),
            ("-.--", "Y"),
            ("--..", "Z"),
            (".----", "1"),
            ("..---", "2"),
            ("...--", "3"),
            ("....-", "4"),
            (".....", "5"),
            ("-....", "6"),
            ("--...", "7"),
            ("---..", "8"),
            ("----.", "9"),
            ("-----", "0"),
        ]
        .iter()
        .map(|x| (x.0.to_string(), x.1.to_string()))
        .collect();

        MorseDecoder { morse_code }
    }

    pub fn decode_morse(&self, encoded: &str) -> String {
        let re = Regex::new(r"[\s]{2,}").unwrap();
        re.split(encoded.trim())
            .map(|c| {
                c.split_whitespace()
                    .map(|l| self.morse_code.get(l).unwrap())
                    .cloned()
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_decodes() {
        let it = MorseDecoder::new().decode_morse("..  --- . ..");
        assert_eq!(it, "I OEI");
    }
}

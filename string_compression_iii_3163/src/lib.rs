pub struct Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        // Collect characters into a vector for easier iteration
        let chars: Vec<char> = word.chars().collect();

        // Initialize variables to track last character and count
        let mut last_count = 1;
        let mut last_char = &chars[0];

        // Build the result string by processing each character in the input
        let mut result = String::new();
        for ch in &chars[1..] { 
            if ch == last_char {
                // If this is the same character as the previous one, increment count
                if last_count < 9 {
                    last_count += 1;
                } else {
                    // If count reaches max value (8), append to result string and reset
                    result.push_str(&format!("{}{}", last_count, last_char));
                    last_count = 1;
                }
            } else {
                // If this is a new character, append previous count and char to result string
                result.push_str(&format!("{}{}", last_count, last_char));
                // Update last character and reset count for next iteration
                last_char = ch;
                last_count = 1;
            }
        }

        // Append the final count and character to the result string
        result.push_str(&format!("{}{}", last_count, last_char));

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("abcde".to_string(), "1a1b1c1d1e".to_string()),
            ("aaaaaaaaaaaaaabb".to_string(), "9a5a2b".to_string()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::compressed_string(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

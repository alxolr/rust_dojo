pub struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        // Collect characters into a vector for easier manipulation
        let chars = s.chars().collect::<Vec<char>>();

        // Initialize variables to track the current run of consecutive characters
        let mut buffer = vec![chars[0]]; // Store the first character separately
        let mut last_char = chars[0]; // Keep track of the previous character
        let mut count = 1; // Count of consecutive occurrences of the same character

        // Iterate over each character in the vector, starting from the second one (index 1)
        for ch in chars.iter().skip(1) {
            // If the current character is the same as the last one, increment the count
            if *ch == last_char {
                count += 1;
            } else {
                // Otherwise, reset the count and update the last character
                count = 1;
                last_char = *ch;
            }

            // Add the current character to the buffer only if it appears less than three times in a row
            if count < 3 {
                buffer.push(*ch);
            }
        }

        // Convert the vector of characters into a string and return the result
        buffer.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![("leeetcode".to_string(), "leetcode".to_string())];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::make_fancy_string(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

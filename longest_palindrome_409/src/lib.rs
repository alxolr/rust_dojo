use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        // Flag to check if we have encountered an odd frequency
        let mut has_odd_frequency = false;
    
        // Create a HashMap to count the frequency of each character in the string
        let char_frequency = s.as_bytes().iter().fold(HashMap::new(), |mut freq_map, char| {
            let count = freq_map.entry(char).or_insert(0);
            *count += 1;
            freq_map
        });
    
        let mut longest_palindrome_length = 0;
    
        // Iterate over the character frequencies
        for &frequency in char_frequency.values() {
            if frequency % 2 == 0 {
                // If the frequency is even, add it to the length of the longest palindrome
                longest_palindrome_length += frequency;
            } else {
                // If the frequency is odd, add (frequency - 1) to the length of the longest palindrome
                // and set the flag has_odd_frequency to true
                longest_palindrome_length += frequency - 1;
                has_odd_frequency = true;
            }
        }
    
        // If there was an odd frequency, add one to the length of the longest palindrome
        // to account for the middle character
        longest_palindrome_length + if has_odd_frequency { 1 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("abccccdd".to_string(), 7),
            ("a".to_string(), 1),
            ("bb".to_string(), 2),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::longest_palindrome(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

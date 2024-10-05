pub struct Solution;

impl Solution {
    // Function to calculate the frequency of each character in a string
    fn frequency(s: &str) -> Vec<usize> {
        s.bytes().fold(vec![0; 26], |mut acc, it| {
            let idx = (it - 'a' as u8) as usize;
            acc[idx] += 1;
            acc
        })
    }

    // Function to check if two frequency vectors are equal
    fn equal_frequencies(s1_freq: &Vec<usize>, s2_freq: &Vec<usize>) -> bool {
        s1_freq == s2_freq
    }

    // Function to check if any permutation of s1 is a substring of s2
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        // Calculate the frequency of characters in s1
        let s1_freq = Self::frequency(&s1);

        // Our goal is to find a substring of specific size s1, that has the same frequency of chars
        let windows_size = s1.len() - 1;

        // We will be expanding the window size by s1 size and try to find this substring
        let mut left = 0;
        let mut right = left + windows_size;

        // Slide the window over s2
        while right < s2.len() {
            // Get the current substring of s2
            let sub_str = &s2[left..=right];
            // Calculate the frequency of characters in the current substring
            let sub_str_freq = Self::frequency(sub_str);

            // Check if the frequency of the current substring matches the frequency of s1
            if Self::equal_frequencies(&s1_freq, &sub_str_freq) {
                return true;
            }

            // Move the window to the right
            left += 1;
            right += 1;
        }

        // If no matching substring is found, return false
        false
    }
}
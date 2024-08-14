pub struct Solution;

impl Solution {
    // Count the frequency of each character in the word
    // sort the frequency in descending order

    pub fn minimum_pushes(word: String) -> i32 {
        let mut freq = [0; 26];
        
        for c in word.chars() {
            freq[(c as u8 - b'a') as usize] += 1;
        }

        freq.sort_unstable_by(|a, b| b.cmp(a));
        
        let mut res = 0;
        let mut sum = 0;

        for i in 0..25 {
            if freq[i] == 0 {
                break;
            }

            sum += freq[i];

            if sum < freq[i + 1] {
                res += sum;
            } else {
                res += freq[i + 1];
            }
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![("abcde".to_string(), 5)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::minimum_pushes(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

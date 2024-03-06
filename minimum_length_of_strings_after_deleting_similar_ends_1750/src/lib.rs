struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes().to_vec();
        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right && s[left] == s[right] {
            let tmp = s[left];

            while left <= right && s[left] == tmp {
                left += 1;
            }

            while left <= right && s[right] == tmp {
                right -= 1;
            }
        }

        right as i32 - left as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("c".to_string(), 1),
            ("ca".to_string(), 2),
            ("cabaabac".to_string(), 0),
            ("aabccabba".to_string(), 3),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::minimum_length(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

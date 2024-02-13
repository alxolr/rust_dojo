struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();

        let mut odd_count = 0;
        let mut even_count: i32 = 0;

        for idx in 0..chars.len() {
            let mut left = idx;
            let mut right = idx;

            odd_count += 1; // we kount size one values

            while left.checked_sub(1).is_some() && right + 1 < chars.len() {
                left -= 1;
                right += 1;

                if chars[left] == chars[right] {
                    odd_count += 1;
                } else {
                    break;
                }
            }
        }

        for idx in 0..chars.len() - 1 {
            let mut left = idx;
            let mut right = idx + 1;

            if chars[left] == chars[right] {
                even_count += 1;

                while left.checked_sub(1).is_some() && right + 1 < chars.len() {
                    left -= 1;
                    right += 1;

                    if chars[left] == chars[right] {
                        even_count += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        odd_count + even_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            // ("abc".to_string(), 3),
            // ("aaa".to_string(), 6),
            // ("hello".to_string(), 6),
            ("xkjkqlajprjwefilxgpdpebieswu".to_string(), 30),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::count_substrings(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

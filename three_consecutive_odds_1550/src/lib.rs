pub struct Solution;

impl Solution {
    pub fn is_three_consecutive_odd(a: &[i32]) -> bool {
        a[0] < a[1] && a[1] < a[2] && a[0] % 2 != 0 && a[1] % 2 != 0 && a[2] % 2 != 0
    }

    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        for idx in 0..arr.len() {
            if idx + 3 < arr.len() {
                if Solution::is_three_consecutive_odd(&arr[idx..idx + 3]) {
                    return true;
                }
            }
        }

        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let scenarios = vec![
            (vec![2, 6, 4, 1], false),
            (vec![1, 2, 34, 3, 4, 5, 7, 23, 12], true),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::three_consecutive_odds(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

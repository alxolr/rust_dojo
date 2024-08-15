pub struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut five_count = 0;
        let mut ten_count = 0;

        for bill in bills {
            match bill {
                5 => {
                    five_count += 1;
                }
                10 => {
                    if !five_count >= 0 {
                        return false;
                    }

                    five_count -= 1;
                    ten_count += 1;
                }
                _ => {
                    if five_count > 0 && ten_count > 0 {
                        five_count -= 1;
                        ten_count -= 1;
                    } else if five_count >= 3 {
                        five_count -= 3;
                    } else {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![10, 10], false),
            (vec![5, 5, 5, 10, 20], true),
            (vec![5, 5, 10, 10, 20], false),
            (vec![20], false),
            (vec![5, 10, 5, 20], true),
            (
                vec![
                    5, 5, 10, 20, 5, 5, 5, 5, 5, 5, 5, 5, 5, 10, 5, 5, 20, 5, 20, 5,
                ],
                true,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::lemonade_change(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

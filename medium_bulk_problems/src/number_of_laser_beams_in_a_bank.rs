pub struct Solution;
impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let (total, _) = bank
            .iter()
            .map(|line| line.chars().filter(|x| x == &'1').count())
            .filter(|item| item != &0)
            .fold((0, 0), |mut acc, item| {
                acc.0 += item * acc.1;
                acc.1 = item;

                acc
            });

        total as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (
                vec![
                    "011001".to_string(),
                    "000000".to_string(),
                    "010100".to_string(),
                    "001000".to_string(),
                ],
                8,
            ),
            (
                vec!["000".to_string(), "111".to_string(), "000".to_string()],
                0,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::number_of_beams(input);
                assert_eq!(result, expected);

                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

use regex::Regex;

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<i32> {
        let operations_pattern: Regex = Regex::new(r#"mul\(\d{1,3},\d{1,3}\)"#)?;
        let operands_pattern =
            Regex::new(r#"mul\((?<left_operand>\d{1,3}),(?<right_operand>\d{1,3})\)"#)?;

        let total_sum: i32 = operations_pattern
            .captures_iter(input)
            .filter_map(|capture| capture.get(0))
            .filter_map(|mul_operation| operands_pattern.captures(mul_operation.as_str()))
            .map(|capture| {
                let left_operand = capture["left_operand"].parse::<i32>().unwrap();
                let right_operand = capture["right_operand"].parse::<i32>().unwrap();

                left_operand * right_operand
            })
            .sum();

        Ok(total_sum)
    }

    pub fn part_2(input: &str) -> Result<i32> {
        let operations_pattern: Regex = Regex::new(r#"mul\(\d{1,3},\d{1,3}\)"#)?;
        let operands_pattern =
            Regex::new(r#"mul\((?<left_operand>\d{1,3}),(?<right_operand>\d{1,3})\)"#)?;

        let total_sum = input
            .split("do()")
            .flat_map(|mut part| {
                if let Some(dont_index) = part.find("don't()") {
                    part = &part[0..dont_index];
                }

                operations_pattern
                    .captures_iter(part)
                    .filter_map(|capture| capture.get(0))
                    .filter_map(|mul_operation| operands_pattern.captures(mul_operation.as_str()))
                    .map(|capture| {
                        let left_operand = capture["left_operand"].parse::<i32>().unwrap();
                        let right_operand = capture["right_operand"].parse::<i32>().unwrap();

                        left_operand * right_operand
                    })
            })
            .sum();

        Ok(total_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(Solution::part_1(input)?, 161);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(Solution::part_2(input)?, 48);

        Ok(())
    }
}

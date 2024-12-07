use crate::error::Result;
pub struct Solution;

impl Solution {
    fn common<F>(input: &str, is_valid: F) -> Result<i64>
    where
        F: Fn(i64, &Vec<i64>) -> bool,
    {
        let parsed_lines = parse_lines(input);

        let total_sum = parsed_lines
            .iter()
            .filter_map(|(target_value, operands)| {
                if is_valid(*target_value, operands) {
                    Some(*target_value)
                } else {
                    None
                }
            })
            .sum::<i64>();

        Ok(total_sum)
    }

    pub fn part_1(input: &str) -> Result<i64> {
        Self::common(input, is_valid_part_1)
    }

    pub fn part_2(input: &str) -> Result<i64> {
        Self::common(input, is_valid_part_2)
    }
}

fn is_valid_part_2(target_value: i64, operands: &Vec<i64>) -> bool {
    let get_accumulated_values = |index: usize, accumulated_value: i64, operand_value| {
        let next_accumulated_values = vec![
            accumulated_value.checked_add(operand_value),
            if index == 0 {
                Some(operand_value)
            } else {
                accumulated_value.checked_mul(operand_value)
            },
            format!("{}{}", accumulated_value, operand_value)
                .parse::<i64>()
                .ok(),
        ];

        next_accumulated_values
    };
    check_accumulated_operands(0, 0, target_value, operands, get_accumulated_values)
}

fn is_valid_part_1(target_value: i64, operands: &Vec<i64>) -> bool {
    let get_accumulated_values = |index: usize, accumulated_value: i64, operand_value| {
        let next_accumulated_values = vec![
            accumulated_value.checked_add(operand_value),
            if index == 0 {
                Some(operand_value)
            } else {
                accumulated_value.checked_mul(operand_value)
            },
        ];

        next_accumulated_values
    };
    check_accumulated_operands(0, 0, target_value, operands, get_accumulated_values)
}

fn check_accumulated_operands<F>(
    index: usize,
    accumulated_value: i64,
    target_value: i64,
    operands: &Vec<i64>,
    get_next_accumulated_values: F,
) -> bool
where
    F: Fn(usize, i64, i64) -> Vec<Option<i64>>,
    F: Clone + Copy,
{
    let next_accumulated_values =
        get_next_accumulated_values(index, accumulated_value, operands[index]);

    if index == operands.len() - 1 {
        next_accumulated_values
            .iter()
            .any(|&value| value == Some(target_value))
    } else {
        next_accumulated_values.iter().any(|&value| {
            value.map_or(false, |v| {
                check_accumulated_operands(
                    index + 1,
                    v,
                    target_value,
                    operands,
                    get_next_accumulated_values,
                )
            })
        })
    }
}

fn parse_lines(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(':');
            let key = parts.next()?.trim().parse::<i64>().ok()?;
            let values: Vec<i64> = parts
                .next()?
                .split_whitespace()
                .filter_map(|value| value.parse::<i64>().ok())
                .collect();

            Some((key, values))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
        assert_eq!(Solution::part_1(input)?, 3749);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
        assert_eq!(Solution::part_2(input)?, 11387);

        Ok(())
    }
}

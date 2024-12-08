use crate::error::Result;
pub struct Solution;

impl Solution {
    // This would be the common part for the both solution where we outline the main idea and algorithm
    // We parse the lines first get the target_value, and the operands that we need to play with
    // We check if our parsed line is valid then we return the target_value to be summed.
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

fn is_valid_part_1(target_value: i64, operands: &Vec<i64>) -> bool {
    // This part is a bit more complex to understand but the idea here is
    // We return a list of possible accumulated values
    let get_accumulated_values = |index: usize, accumulated_value: i64, operand_value| {
        // We return an options of possible next values cause we need to handle overflows
        let possible_accumulated_values = vec![
            // For the add operator, we get the already accumulated and the current operand value
            accumulated_value.checked_add(operand_value),
            // For the multiplication, we need to check for the first one to not have multiplication by zero
            // So if it's the first one we just return the current operand value
            if index == 0 {
                Some(operand_value)
            } else {
                accumulated_value.checked_mul(operand_value)
            },
        ];

        possible_accumulated_values
    };

    // We call the recursive function, starting at index 0, and accumulator zero
    check_accumulated_operands(0, 0, target_value, operands, get_accumulated_values)
}

fn is_valid_part_2(target_value: i64, operands: &Vec<i64>) -> bool {
    let get_accumulated_values = |index: usize, accumulated_value: i64, operand_value| {
        let possible_accumulated_values = vec![
            accumulated_value.checked_add(operand_value),
            if index == 0 {
                Some(operand_value)
            } else {
                accumulated_value.checked_mul(operand_value)
            },
            // We add the concatenation posibility
            format!("{}{}", accumulated_value, operand_value)
                .parse::<i64>()
                .ok(),
        ];

        possible_accumulated_values
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

    // This is the finish line, when our index is the last
    // We check if any of the accumulated values is equal to our target
    if index == operands.len() - 1 {
        next_accumulated_values
            .iter()
            .any(|&value| value == Some(target_value))
    } else {
        // In the other cases we go recursively, by checking all possible operators
        // and we increment the index
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

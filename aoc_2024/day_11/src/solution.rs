use std::collections::HashMap;

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn common(input: &str, blinks: u64) -> Result<u64> {
        let stones: Vec<u64> = input
            .lines()
            .flat_map(|line| {
                line.split_whitespace()
                    .flat_map(|number| number.parse::<u64>())
            })
            .collect();

        let mut total_stones = 0;
        let mut memo = HashMap::new();

        for stone in stones {
            total_stones += recursive_blink(&mut memo, stone, blinks)
        }

        Ok(total_stones)
    }

    pub fn part_1(input: &str) -> Result<u64> {
        let total_stones = Self::common(input, 25)?;

        Ok(total_stones)
    }

    pub fn part_2(input: &str) -> Result<u64> {
        let total_stones = Self::common(input, 75)?;

        Ok(total_stones)
    }
}

fn recursive_blink(memo: &mut HashMap<(u64, u64), u64>, stone: u64, blinks: u64) -> u64 {
    // If we already computed a sub solution then just return the answer
    if let Some(result) = memo.get(&(stone, blinks)) {
        return *result;
    }

    if blinks > 0 {
        // We apply the rules in recursive manner
        let stones_sum = match stone {
            0 => recursive_blink(memo, 1, blinks - 1),
            x if is_even(x) => {
                let (left, right) = split_two(x).unwrap();

                recursive_blink(memo, left, blinks - 1) + recursive_blink(memo, right, blinks - 1)
            }
            _ => recursive_blink(memo, stone * 2024, blinks - 1),
        };

        // We store the computed result in a memo table
        memo.insert((stone, blinks), stones_sum);

        stones_sum
    } else {
        // base case scenario we return one if the are no more blinks
        return 1;
    }
}

fn is_even(num: u64) -> bool {
    num.to_string().len() % 2 == 0
}

fn split_two(num: u64) -> Result<(u64, u64)> {
    let num = num.to_string();
    let len = num.len();

    let left = &num[0..len / 2].parse::<u64>().unwrap_or(0);
    let right = &num[len / 2..len].parse::<u64>().unwrap_or(0);

    Ok((*left, *right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_two_ok() -> Result<()> {
        let num = 1000;
        let (left, right) = split_two(num)?;

        assert_eq!(left, 10);
        assert_eq!(right, 0);

        let num = 1121;
        let (left, right) = split_two(num)?;

        assert_eq!(left, 11);
        assert_eq!(right, 21);

        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"125 17"#;
        assert_eq!(Solution::part_1(input)?, 55312);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"125 17"#;
        assert_eq!(Solution::part_2(input)?, 65601038650482);

        Ok(())
    }
}

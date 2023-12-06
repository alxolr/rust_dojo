use regex::Regex;

use crate::error::Result;

pub struct Solution;

impl Solution {
    pub fn process(input: &str, strategy: fn(&str) -> Result<Vec<(u64, u64)>>) -> Result<u64> {
        let response = strategy(input)?
            .into_iter()
            .map(winning_combos)
            .fold(1, |mut acc, item| {
                acc *= item;
                acc
            });
        Ok(response)
    }

    pub fn part_2(input: &str) -> Result<u64> {
        Solution::process(input, normalize_input_part_2)
    }

    pub fn part_1(input: &str) -> Result<u64> {
        Solution::process(input, normalize_input_part_1)
    }
}

fn winning_combos(pair: (u64, u64)) -> u64 {
    // winning condition strictly greater distance
    let (time, distance) = pair;

    let mut winners = 0;

    for pressed_time in 1..time {
        let speed = pressed_time;
        let time_remaining = time - pressed_time;

        let potential_distance = time_remaining * speed;
        if potential_distance > distance {
            winners += 1;
        }
    }

    winners
}

fn normalize_input_part_1(input: &str) -> Result<Vec<(u64, u64)>> {
    let [times, distances]: [Vec<u64>; 2] = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(numbers_from_line)
        .flatten()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let result: Vec<(u64, u64)> = times.into_iter().zip(distances.into_iter()).collect();

    Ok(result)
}

fn normalize_input_part_2(input: &str) -> Result<Vec<(u64, u64)>> {
    let [time, distance]: [u64; 2] = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(single_number)
        .flatten()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    Ok(vec![(time, distance)])
}

fn single_number(line: &str) -> Result<u64> {
    let numbers_re = Regex::new(r"(\d+)").unwrap();

    let number = numbers_re
        .find_iter(line)
        .map(|nums| nums.as_str())
        .filter(|n| !n.is_empty())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    Ok(number)
}

fn numbers_from_line(line: &str) -> Result<Vec<u64>> {
    let numbers_re = Regex::new(r"(\d+)").unwrap();

    let numbers = numbers_re
        .find_iter(line)
        .map(|num| num.as_str().parse::<u64>())
        .flatten()
        .collect::<Vec<u64>>();

    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r"Time:      7  15   30
        Distance:  9  40  200
        "
        .to_string()
    }

    #[test]
    fn test_single_number_from_line_ok() -> Result<()> {
        let line = "Time:      7  15   30";
        let expected = 71530;

        assert_eq!(single_number(line)?, expected);

        Ok(())
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 288;
        let input = example();
        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }

    #[test]
    fn test_winning_combos_ok() {
        let pair = (7, 9);

        assert_eq!(winning_combos(pair), 4);
    }

    #[test]
    fn test_normalize_input_part_2_from_example_ok() -> Result<()> {
        let input = example();
        let expected = vec![(71530, 940200)];

        let normalized = normalize_input_part_2(&input)?;

        assert_eq!(expected, normalized);

        Ok(())
    }

    #[test]
    fn test_normalize_input_part_1_from_example_ok() -> Result<()> {
        let input = example();
        let expected = vec![(7, 9), (15, 40), (30, 200)];

        let normalized = normalize_input_part_1(&input)?;

        assert_eq!(expected, normalized);

        Ok(())
    }

    #[test]
    fn test_parse_numbers_from_line_ok() -> Result<()> {
        let line = "Time:      7  15   30";
        let expected_numbers = vec![7, 15, 30];

        assert_eq!(numbers_from_line(line)?, expected_numbers);

        Ok(())
    }
}

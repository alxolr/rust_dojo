use std::collections::HashMap;

use regex::Regex;

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<i64> {
        let games = parse_input(input);

        let sum = games
            .into_iter()
            .flat_map(|(a, b, sum)| solve_game(a, b, sum))
            .sum::<i64>();

        Ok(sum)
    }

    pub fn part_2(input: &str) -> Result<i64> {
        let games = parse_input(input);
        let big_number = 10000000000000;

        let sum = games
            .into_iter()
            .flat_map(|(a, b, (sx, sy))| solve_game(a, b, (sx + big_number, sy + big_number)))
            .sum::<i64>();

        Ok(sum)
    }
}

fn solve_game(a: (i64, i64), b: (i64, i64), sum: (i64, i64)) -> Option<i64> {
    let b_count = (a.1 * sum.0 - a.0 * sum.1) / (a.1 * b.0 - a.0 * b.1);
    if b_count < 0 || (a.1 * sum.0 - a.0 * sum.1) % (a.1 * b.0 - a.0 * b.1) != 0 {
        return None;
    }

    let a_count = (sum.1 - b.1 * b_count) / a.1;
    if a_count < 0 || (sum.1 - b.1 * b_count) % a.1 != 0 {
        return None;
    }

    Some((a_count * 3) + b_count)
}

fn parse_input(input: &str) -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    input
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines();

            let button_a = extract_button(lines.next());
            let button_b = extract_button(lines.next());
            let total_sum = extract_prize(lines.next());

            (button_a, button_b, total_sum)
        })
        .collect()
}

fn extract_coordinates(input: Option<&str>, pattern: Regex) -> (i64, i64) {
    match input {
        Some(input) => {
            if let Some(captures) = pattern.captures(input) {
                let x = &captures["x"].parse::<i64>().unwrap();
                let y = &captures["y"].parse::<i64>().unwrap();

                return (*x, *y);
            }

            panic!("Could not match pattern");
        }
        None => panic!("Invalid input string"),
    }
}

fn extract_button(input: Option<&str>) -> (i64, i64) {
    let re = Regex::new(r#"X\+(?<x>\d+),\sY\+(?<y>\d+)"#).expect("Invalid regex Pattern");
    extract_coordinates(input, re)
}

fn extract_prize(input: Option<&str>) -> (i64, i64) {
    let re = Regex::new(r#"X=(?<x>\d+),\sY=(?<y>\d+)"#).expect("Invalid regex Pattern");
    extract_coordinates(input, re)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
        assert_eq!(Solution::part_1(input)?, 480);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#""#;
        assert_eq!(Solution::part_2(input)?, 0);

        Ok(())
    }
}

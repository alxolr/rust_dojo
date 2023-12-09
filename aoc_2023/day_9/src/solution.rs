use regex::Regex;

use crate::error::Result;

pub struct Solution;

impl Solution {
    pub fn process(input: &str, strat: fn(Vec<i32>) -> i32) -> Result<i32> {
        let result = input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .flat_map(numbers)
            .map(strat)
            .sum::<i32>();

        Ok(result)
    }

    pub fn part_1(input: &str) -> Result<i32> {
        Solution::process(input, next_sequence)
    }

    pub fn part_2(input: &str) -> Result<i32> {
        Solution::process(input, prev_sequence)
    }
}

fn numbers(line: &str) -> Result<Vec<i32>> {
    let regex = Regex::new(r"([-\d]+)").unwrap();

    let result = regex
        .find_iter(line)
        .flat_map(|x| x.as_str().parse::<i32>())
        .collect::<Vec<i32>>();

    Ok(result)
}

fn gen_seq(input: Vec<i32>, capture: fn(&Vec<i32>) -> i32) -> Vec<i32> {
    let mut captures = vec![];

    let mut curr = input;
    loop {
        curr = curr
            .windows(2)
            .map(|item| {
                let [l, r]: [i32; 2] = item.try_into().unwrap();

                r - l
            })
            .collect();

        captures.push(capture(&curr));

        if curr.iter().all(|x| *x == 0) {
            break;
        }
    }

    captures
}

fn prev_sequence(input: Vec<i32>) -> i32 {
    let first_input = input[0];
    let mut firsts = gen_seq(input, |item: &Vec<i32>| *item.first().unwrap());

    firsts.reverse();

    loop {
        let first = firsts[0];
        let second = firsts[1];

        let folded_val = second - first;
        let skipped = firsts.into_iter().skip(2).collect::<Vec<i32>>();

        firsts = vec![vec![folded_val], skipped].concat();

        if firsts.len() == 1 {
            break;
        }
    }

    let pre_last: i32 = firsts[0];

    first_input - pre_last
}

fn next_sequence(input: Vec<i32>) -> i32 {
    let last_input = input[input.len() - 1];
    let lasts = gen_seq(input, |item| *item.last().unwrap());
    let pre_last = lasts.iter().sum::<i32>();

    last_input + pre_last
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r"0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
        "
        .to_string()
    }

    #[test]
    fn test_next_sequence_ok() {
        let scenarios = vec![
            (vec![0, 3, 6, 9, 12, 15], 18),
            (vec![1, 3, 6, 10, 15, 21], 28),
            (vec![10, 13, 16, 21, 30, 45], 68),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = next_sequence(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn test_numbers_parse_minus_ok() -> Result<()> {
        let line = "-1 -23 32 345 13213";
        let expected = vec![-1, -23, 32, 345, 13213];

        assert_eq!(numbers(line)?, expected);

        Ok(())
    }

    #[test]
    fn test_solution_part_2_from_example() -> Result<()> {
        let expected = 2;
        let input = example();
        assert_eq!(Solution::part_2(&input)?, expected);

        Ok(())
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 114;
        let input = example();
        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }
}

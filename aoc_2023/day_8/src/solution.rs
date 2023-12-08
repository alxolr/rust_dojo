use num::integer::lcm;
use std::collections::HashMap;

use regex::Regex;

use crate::error::{self, Result};

pub struct Solution;

#[derive(Debug, PartialEq)]
pub struct Desert {
    path: Vec<char>,
    network: HashMap<String, (String, String)>,
}

impl Solution {
    pub fn part_2(input: &str) -> Result<u64> {
        let desert: Desert = input.try_into()?;

        let starts = desert
            .network
            .keys()
            .filter(|key| key.ends_with('A'))
            .map(|k| k.as_str())
            .collect::<Vec<_>>();

        let distances = starts
            .iter()
            .map(|start| Solution::count_distance(start, &desert, |s| s.ends_with('Z')))
            .collect::<Vec<u64>>();

        let result = distances.iter().fold(1u64, |acc, item| lcm(acc, *item));

        Ok(result)
    }

    pub fn part_1(input: &str) -> Result<u64> {
        let desert: Desert = input.try_into()?;

        let steps = Solution::count_distance("AAA", &desert, |curr| curr == "ZZZ");

        Ok(steps)
    }

    fn count_distance(start: &str, desert: &Desert, strategy: fn(&str) -> bool) -> u64 {
        let mut steps = 0;
        let mut curr = start;
        let mut path_iter = desert.path.iter().cycle();

        loop {
            let direction = path_iter.next().unwrap();
            let value = desert.network.get(curr).unwrap();

            curr = match direction {
                'L' => {
                    let (left, _) = value;

                    left.as_str()
                }
                'R' => {
                    let (_, right) = value;

                    right.as_str()
                }
                _ => panic!("Unsupported direction"),
            };
            steps += 1;

            if strategy(curr) {
                break;
            }
        }

        steps
    }
}

impl TryFrom<&str> for Desert {
    type Error = error::Error;

    fn try_from(value: &str) -> Result<Self> {
        let (networks, path): (Vec<&str>, Vec<&str>) = value
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .partition(|l| l.contains('='));

        let path = path.first().unwrap_or(&"").chars().collect::<Vec<char>>();

        let network = networks
            .iter()
            .map(|line| {
                let regex = Regex::new(r"([A-Z]{3})+").unwrap();

                let [source, left, right]: [String; 3] = regex
                    .find_iter(line)
                    .map(|m: regex::Match<'_>| m.as_str().to_string())
                    .collect::<Vec<String>>()
                    .try_into()
                    .unwrap();

                (source, (left, right))
            })
            .collect::<HashMap<String, (String, String)>>();

        Ok(Self { path, network })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r"RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
        "
        .to_string()
    }

    fn example_part_1() -> String {
        r"LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
        "
        .to_string()
    }

    fn example_part_2() -> String {
        r"LR

        FTA = (FTB, XXX)
        FTB = (XXX, FTZ)
        FTZ = (FTB, XXX)
        FXA = (FXB, XXX)
        FXB = (XDC, XDC)
        XDC = (FXZ, FXZ)
        FXZ = (FXB, FXB)
        XXX = (XXX, XXX)"
            .to_string()
    }

    #[test]
    fn test_desert_from_input_ok() -> Result<()> {
        let input = example();

        let vec = vec![
            ("AAA", "BBB", "CCC"),
            ("BBB", "DDD", "EEE"),
            ("CCC", "ZZZ", "GGG"),
            ("DDD", "DDD", "DDD"),
            ("GGG", "GGG", "GGG"),
            ("EEE", "EEE", "EEE"),
            ("ZZZ", "ZZZ", "ZZZ"),
        ]
        .iter()
        .map(|(source, left, right)| (source.to_string(), (left.to_string(), right.to_string())))
        .collect::<Vec<_>>();

        let network = HashMap::from_iter(vec.into_iter());

        let expected_desert = Desert {
            path: vec!['R', 'L'],
            network,
        };

        let desert: Desert = input.as_str().try_into()?;

        assert_eq!(desert, expected_desert);

        Ok(())
    }

    #[test]
    fn test_solution_part_2_from_example() -> Result<()> {
        let expected = 6;
        let input = example_part_2();
        assert_eq!(Solution::part_2(&input)?, expected);

        Ok(())
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 6;
        let input = example_part_1();
        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }
}

use std::collections::HashMap;

use regex::Regex;

use crate::error::Result;

pub struct Solution;

#[derive(Debug, PartialEq)]
pub struct Desert {
    path: Vec<char>,
    network: HashMap<String, (String, String)>,
}

impl Solution {
    pub fn part_2(input: &str) -> Result<u64> {
        let result = 1;
        Ok(result)
    }

    pub fn part_1(input: &str) -> Result<u64> {
        Ok(10)
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

    #[test]
    fn test_solution_part_2_from_example() -> Result<()> {
        let expected = 1;
        let input = example();
        assert_eq!(Solution::part_2(&input)?, expected);

        Ok(())
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 1;
        let input = example();
        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }
}

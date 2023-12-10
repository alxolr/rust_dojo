use regex::Regex;

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<i32> {
        Ok(1)
    }

    pub fn part_2(input: &str) -> Result<i32> {
        Ok(1)
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

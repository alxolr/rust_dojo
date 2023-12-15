use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn process(input: &str) -> Result<i64> {
        Ok(1)
    }

    pub fn part_1(input: &str) -> Result<i64> {
        Solution::process(input)
    }

    pub fn part_2(input: &str) -> Result<i64> {
        Solution::process(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
   
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
        "
        .to_string()
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 405;
        let input = example();

        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }
}

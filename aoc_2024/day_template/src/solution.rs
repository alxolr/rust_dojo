use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(_input: &str) -> Result<i32> {
        Ok(0)
    }

    pub fn part_2(_input: &str) -> Result<i32> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = "";
        assert_eq!(super::Solution::part_1(input)?, 0);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = "";
        assert_eq!(super::Solution::part_2(input)?, 0);

        Ok(())
    }
}

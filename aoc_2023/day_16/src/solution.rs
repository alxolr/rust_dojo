use crate::error::Result;
pub struct Solution;

type Len = (String, i32);

impl Solution {
    pub fn part_1(input: &str) -> Result<i64> {
        Ok(1)
    }

    pub fn part_2(input: &str) -> Result<i32> {
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|...."
            .to_string()
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 1320;
        let input = example();

        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }

    #[test]
    fn test_solution_part_2_from_example() -> Result<()> {
        let expected = 145;
        let input = example();

        assert_eq!(Solution::part_2(&input)?, expected);

        Ok(())
    }
}

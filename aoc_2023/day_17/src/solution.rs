use std::collections::{HashMap, HashSet};

use crate::error::Result;
pub struct Solution;

type Point = (isize, isize);
type DirectedPoint<'a> = (Point, &'a Dir);

pub struct Grid {
    data: HashMap<Point, char>,
    rows: isize,
    cols: isize,
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Solution {

    pub fn part_1(input: &str) -> Result<i64> {
        Ok(1)
    }

    pub fn part_2(input: &str) -> Result<i64> {
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r"2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533"
            .to_string()
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 102;
        let input = example();

        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }

    // #[test]
    // fn test_solution_part_2_from_example() -> Result<()> {
    //     let expected = 51;
    //     let input = example();

    //     assert_eq!(Solution::part_2(&input)?, expected);

    //     Ok(())
    // }
}

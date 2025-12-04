use rustc_hash::FxHashSet;

const ADJACENT_DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

use crate::error::Result;
pub struct Solution;
impl Solution {
    pub fn part_1(input: &str) -> Result<i32> {
        let grid = parse_grid(input);

        Ok(grid
            .iter()
            .flat_map(|point| Self::solution(&grid, point))
            .count() as i32)
    }

    fn solution(
        papers: &FxHashSet<(usize, usize)>,
        point: &(usize, usize),
    ) -> Option<(usize, usize)> {
        let neighbours_count = ADJACENT_DELTAS
            .iter()
            .flat_map(|&(dr, dc)| get_neighbour(point.0, dr, point.1, dc))
            .filter(|neighbour| papers.contains(neighbour))
            .count();

        match neighbours_count {
            count if count < 4 => Some(*point),
            _ => None,
        }
    }

    pub fn part_2(input: &str) -> Result<i32> {
        let mut grid = parse_grid(input);
        let mut answer = 0;

        loop {
            let papers_to_clean = grid
                .iter()
                .flat_map(|point| Self::solution(&grid, point))
                .collect::<FxHashSet<(usize, usize)>>();

            if papers_to_clean.is_empty() {
                break;
            }

            answer += papers_to_clean.len();
            grid = grid.difference(&papers_to_clean).cloned().collect()
        }

        Ok(answer as i32)
    }
}

fn parse_grid(input: &str) -> FxHashSet<(usize, usize)> {
    let mut papers = FxHashSet::default();

    for (row, line) in input.lines().map(|line| line.trim()).enumerate() {
        papers.extend(
            line.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'@')
                .map(|(col, _)| (row, col)),
        );
    }

    papers
}

fn get_neighbour(row: usize, dr: isize, col: usize, dc: isize) -> Option<(usize, usize)> {
    match (row as isize + dr, col as isize + dc) {
        (row, col) if row >= 0 && col >= 0 => Some((row as usize, col as usize)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@."#;
        assert_eq!(Solution::part_1(input)?, 13);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@."#;
        assert_eq!(Solution::part_2(input)?, 43);

        Ok(())
    }
}

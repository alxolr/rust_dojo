use std::collections::{HashSet, VecDeque};

use crate::error::Result;
pub struct Solution;

const SPLITTER: char = '^';
const START: char = 'S';
const BEAM: char = '|';

type Beam = (isize, isize);

impl Solution {
    // Idea for part 1, is to do a breadth for search and emit beams either down or split them
    // Cause some beams are splitted and overlap to not overcompute them we have another hashset for distinct beams
    // In the end we will check how many of the distinct_beams hit the splitters
    pub fn part_1(input: &str) -> Result<u64> {
        let grid = parse_grid(input);
        let distinct_beams = simulate_run(&grid);

        let splitters_count = distinct_beams
            .iter()
            .filter(|beam| is_splitter(&grid, beam))
            .count() as u64;

        Ok(splitters_count as u64)
    }

    /// Idea for part 2:
    /// We will run once the part 1 and gather all the beams coordinates from distinct_beams
    /// We will apply the beams on the grid
    /// We will go backwards from leafs and mark each branch as 1
    /// Where we find a splitter it will be the sum of the branches
    /// .  .  .  .  .  .  .  40 .  .  .  .  .  .  .
    /// .  .  .  .  .  .  .  40 .  .  .  .  .  .  .
    /// .  .  .  .  .  .  .  40 .  .  .  .  .  .  .
    /// .  .  .  .  .  . 25  .  15 .  .  .  .  .  .
    /// .  .  .  .  .  . 25  .  15 .  .  .  .  .  .
    /// .  .  .  .  .  17  . 8  .  7  .  .  .  .  .
    /// .  .  .  .  .  17  . 8  .  7  .  .  .  .  .
    /// .  .  .  .  10  . 7  .  1  .  6  .  .  .  .
    /// .  .  .  .  10 .  7  .  1  .  6  .  .  .  .
    /// .  .  .  5  .  5  .  2  1  2  .  4  .  .  .
    /// .  .  .  5  .  5  .  2  1  2  .  4  .  .  .
    /// .  .  4  .  1  .  4  2  1  .  1  .  3  .  .
    /// .  .  4  .  1  .  4  2  1  .  1  .  3  .  .
    /// .  2  .  2  1  2  .  2  1  .  1  1  .  2  .
    /// .  2  .  2  1  2  .  2  1  2  1  1  .  2  .
    /// 1  .  1  .  1  .  1  .  1  .  1  1  1  .  1
    pub fn part_2(input: &str) -> Result<u64> {
        let grid = parse_grid(input);
        let rows = grid.len();
        let cols = grid[0].len();

        // A dp grid
        let mut timelines_dp = vec![vec![0; cols]; rows];
        let (start_row, start_col) = locate_start(&grid).unwrap_or((0, 0));

        // Initialize the leaf row where there are the last beams with 1
        for col in 0..cols {
            timelines_dp[rows - 1][col] = 1;
        }

        for row in (0..rows - 1).rev() {
            for col in 0..cols {
                match grid[row][col] {
                    SPLITTER => {
                        // take the sum of the values from bellow left and right
                        timelines_dp[row][col] =
                            timelines_dp[row + 1][col - 1] + timelines_dp[row + 1][col + 1];
                    }
                    _ => {
                        timelines_dp[row][col] = timelines_dp[row + 1][col];
                    }
                }
            }
        }

        let timelines_count = timelines_dp[(start_row) as usize][start_col as usize];

        Ok(timelines_count)
    }
}

fn simulate_run(grid: &Vec<Vec<char>>) -> HashSet<Beam> {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let (start_row, start_col) = locate_start(&grid).unwrap_or((0, 0));
    let mut beams = VecDeque::new(); // Breath First Search

    // Here we will store the coordinates of all beams
    let mut distinct_beams = HashSet::new();

    beams.push_front((start_row + 1, start_col)); // We are emiting a beam from start going down
    distinct_beams.insert((start_row + 1, start_col));

    // We go line by line using breadth first search
    while let Some(beam) = beams.pop_front() {
        // if we hit a splitter then we need to split the beam
        if is_splitter(&grid, &beam) {
            // The shinanigans needed to handle possible out of bounds on the left, right or bottom
            match split_beam(beam, rows, cols) {
                (Some(left), Some(right)) => {
                    if !distinct_beams.contains(&left) {
                        distinct_beams.insert(left);
                        beams.push_back(left);
                    }
                    if !distinct_beams.contains(&right) {
                        distinct_beams.insert(right);
                        beams.push_back(right);
                    }
                }
                (Some(left), None) => {
                    if !distinct_beams.contains(&left) {
                        distinct_beams.insert(left);
                        beams.push_back(left);
                    }
                }
                (None, Some(right)) => {
                    if !distinct_beams.contains(&right) {
                        distinct_beams.insert(right);
                        beams.push_back(right);
                    }
                }
                (_, _) => {}
            }
        } else {
            if let Some(down) = move_down(beam, rows, cols) {
                if !distinct_beams.contains(&down) {
                    beams.push_back(down);
                    distinct_beams.insert(down);
                }
            }
        }
    }

    distinct_beams
}

fn is_splitter(grid: &Vec<Vec<char>>, beam: &Beam) -> bool {
    if grid[beam.0 as usize][beam.1 as usize] == SPLITTER {
        true
    } else {
        false
    }
}

fn move_down(beam: Beam, rows: isize, cols: isize) -> Option<Beam> {
    let (row, col) = beam;
    if row + 1 >= rows || row < 0 || col < 0 || col > cols {
        None
    } else {
        Some((row + 1, col))
    }
}

// Split the beam into two, handle the out of bounds
fn split_beam(beam: Beam, rows: isize, cols: isize) -> (Option<Beam>, Option<Beam>) {
    let (row, col) = beam;

    let left = if row + 1 >= rows || row < 0 || col - 1 < 0 {
        None
    } else {
        Some((row + 1, col - 1))
    };

    let right = if row + 1 >= rows || row < 0 || col + 1 >= cols {
        None
    } else {
        Some((row + 1, col + 1))
    };

    (left, right)
}

fn locate_start(grid: &Vec<Vec<char>>) -> Option<Beam> {
    for (row_id, row) in grid.iter().enumerate() {
        for (col_id, item) in row.iter().enumerate() {
            if *item == START {
                return Some((row_id as isize, col_id as isize));
            }
        }
    }

    None
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMON_INPUT: &str = r#".......S.......
    ...............
    .......^.......
    ...............
    ......^.^......
    ...............
    .....^.^.^.....
    ...............
    ....^.^...^....
    ...............
    ...^.^...^.^...
    ...............
    ..^...^.....^..
    ...............
    .^.^.^.^.^...^.
    ...............
"#;

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(Solution::part_1(COMMON_INPUT)?, 21);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(Solution::part_2(COMMON_INPUT)?, 40);

        Ok(())
    }
}

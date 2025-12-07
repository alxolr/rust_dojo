use std::collections::{HashSet, VecDeque};

use crate::error::Result;
pub struct Solution;

const SPLITTER: char = '^';
const START: char = 'S';

type Beam = (isize, isize);

impl Solution {
    pub fn part_1(input: &str) -> Result<u64> {
        let grid = parse_grid(input);
        let rows = grid.len() as isize;
        let cols = grid[0].len() as isize;

        let (start_row, start_col) = locate_start(&grid).unwrap_or((0, 0));
        let mut beams = VecDeque::new(); // Breath first search

        beams.push_front((start_row + 1, start_col)); // we are emiting a beam from start going down
        let mut distinct_splitters = HashSet::new();
        let mut visited = HashSet::new();

        while let Some(beam) = beams.pop_front() {
            // handle the out of bounds
            if beam.0 > rows || beam.1 < 0 || beam.1 > cols {
                continue;
            }

            //check if current beam is not hitting a splitter
            if hit_splitter(&grid, beam) {
                distinct_splitters.insert(beam);
                let (left, right) = split_beam(beam);

                if !visited.contains(&left) {
                    visited.insert(left);
                    beams.push_back(left);
                }
                if !visited.contains(&right) {
                    visited.insert(right);
                    beams.push_back(right);
                }
            } else {
                let down = move_down(beam);
                if !visited.contains(&down) {
                    beams.push_back(down);
                    visited.insert(down);
                }
            }
        }

        let splitters_count = distinct_splitters.len();

        Ok(splitters_count as u64)
    }

    pub fn part_2(_input: &str) -> Result<i32> {
        Ok(0)
    }
}

fn hit_splitter(grid: &Vec<Vec<char>>, beam: Beam) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    if beam.0 < 0 || beam.0 >= rows as isize {
        return false;
    }

    if beam.1 < 0 || beam.1 >= cols as isize {
        return false;
    }

    if grid[beam.0 as usize][beam.1 as usize] == SPLITTER {
        true
    } else {
        false
    }
}

fn move_down(beam: Beam) -> Beam {
    (beam.0 + 1, beam.1)
}

fn split_beam(beam: Beam) -> (Beam, Beam) {
    ((beam.0 + 1, beam.1 - 1), (beam.0 + 1, beam.1 + 1))
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

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        println!();
        for item in row {
            print!("{:}", item);
        }
    }

    println!()
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
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#".......S.......
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
        assert_eq!(Solution::part_1(input)?, 21);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#""#;
        assert_eq!(Solution::part_2(input)?, 0);

        Ok(())
    }
}

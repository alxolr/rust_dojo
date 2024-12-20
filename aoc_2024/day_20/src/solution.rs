use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::error::Result;
pub struct Solution;

pub type Position = (usize, usize);

#[derive(PartialEq, Eq)]
pub struct State {
    cost: u32,
    pos: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Solution {
    pub fn part_1(input: &str) -> Result<u32> {
        let mut grid = load_grid(input);

        let start = find_position(&grid, 'S')?;
        let end = find_position(&grid, 'E')?;
        let race_track = shortest_path(start, end, &grid).unwrap_or(vec![]);
        let base_cost = race_track.len();

        let cheats = generate_cheats(&grid, &race_track);

        let cheat_results = cheats
            .into_iter()
            .map(|cheat| {
                let temp = grid[cheat.0][cheat.1];
                grid[cheat.0][cheat.1] = '.';
                let new_path = shortest_path(start, end, &grid).unwrap_or_default();
                grid[cheat.0][cheat.1] = temp;

                base_cost - new_path.len()
            })
            .fold(HashMap::new(), |mut acc, it| {
                let entry = acc.entry(it).or_insert(0);
                *entry += 1;

                acc
            });

        let cheats_count = cheat_results
            .iter()
            .filter_map(
                |(save, count)| {
                    if *save >= 100 {
                        Some(*count)
                    } else {
                        None
                    }
                },
            )
            .sum::<u32>();

        Ok(cheats_count)
    }

    pub fn part_2(_input: &str) -> Result<i32> {
        Ok(0)
    }
}

fn generate_cheats(grid: &[Vec<char>], track: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let rows: usize = grid.len();
    let cols = grid[0].len();
    // We need to find all obsticles that have link to the track on any of the side and try to disable them

    let mut cheats = vec![];
    // Avoid first and last row as it's boundary
    for row in 1..rows - 1 {
        // Avoid first and last column as it's boundaries
        for col in 1..cols - 1 {
            if grid[row][col] == '#' {
                // The key moment in here is that a cheat needs to start and end on the track passing through walls 
                // This means this wall needs to be bounded on the track by at least two faces
                let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

                if directions
                    .into_iter()
                    .map(|(dx, dy)| {
                        let new_row = row as isize - dx;
                        let new_col = col as isize - dy;

                        track.contains(&(new_row as usize, new_col as usize))
                    })
                    .count()
                    > 2
                {
                    cheats.push((row, col));
                }
            }
        }
    }

    cheats
}

fn find_position(grid: &[Vec<char>], needle: char) -> Result<Position> {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == needle {
                return Ok((row_idx, col_idx));
            }
        }
    }

    panic!("Needle not found in the grid");
}

fn shortest_path(
    start: Position,
    end: Position,
    grid: &[Vec<char>],
) -> Option<Vec<(usize, usize)>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut cost_grid = vec![vec![u32::MAX; cols]; rows];
    let mut parents: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; cols]; rows];

    cost_grid[start.0][start.1] = 0;
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(State {
        cost: 0,
        pos: start,
    });

    while let Some(current_state) = priority_queue.pop() {
        let (current_row, current_col) = current_state.pos;

        if current_state.pos == end {
            break;
        }

        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (d_row, d_col) in directions {
            let new_row = current_row as isize + d_row;
            let new_col = current_col as isize + d_col;

            if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
                let next_row: usize = new_row as usize;
                let next_col = new_col as usize;
                if ['.', 'E'].contains(&grid[next_row][next_col]) {
                    let new_cost = current_state.cost + 1;

                    if new_cost < cost_grid[next_row][next_col] {
                        cost_grid[next_row][next_col] = new_cost;
                        parents[next_row][next_col] = Some((current_row, current_col));
                        priority_queue.push(State {
                            cost: new_cost,
                            pos: (next_row, next_col),
                        });
                    }
                }
            }
        }
    }

    let mut path = Vec::new();
    let mut current: (usize, usize) = end;

    while let Some(parent) = parents[current.0][current.1] {
        path.push(current);
        current = parent;
    }
    path.reverse();

    if !path.is_empty() {
        Some(path)
    } else {
        None
    }
}

fn load_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
        assert_eq!(Solution::part_1(input)?, 0);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#""#;
        assert_eq!(Solution::part_2(input)?, 0);

        Ok(())
    }
}

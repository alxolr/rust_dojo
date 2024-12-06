use std::collections::HashSet;

use crate::error::Result;
pub struct Solution;

type Cell = (usize, usize);
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Solution {
    pub fn part_1(input: &str) -> Result<i32> {
        let mut grid = load_grid(input);
        // Store the visited cells in a HashSet to count the distinct ones
        let mut visited_cells = HashSet::<Cell>::new();

        // Locate the guardian's starting position
        let start_position = guardian_start_position(&grid);

        // Remove the guardian from the grid, in case it returns from a different direction
        grid[start_position.0][start_position.1] = '.';

        // Add the starting position to visited cells
        visited_cells.insert(start_position);

        // Set the initial direction of the guardian to be Up
        let mut current_direction = Direction::Up;

        // Simulate the traversal
        let (mut row, mut col) = start_position;
        loop {
            visited_cells.insert((row, col));
            // check the
            match current_direction {
                Direction::Up => {
                    if row.checked_sub(1).is_some() {
                        match grid[row - 1][col] {
                            '.' => row -= 1,                   // Move up
                            _ => current_direction = Direction::Right, // Change direction
                        }
                    } else {
                        // This is the exit from the grid means we are out
                        break;
                    }
                }
                Direction::Right => {
                    if col + 1 < grid[0].len() {
                        match grid[row][col + 1] {
                            '.' => col += 1,                  // Move Right
                            _ => current_direction = Direction::Down, // change direction
                        }
                    } else {
                        // Exit from the grid
                        break;
                    }
                }
                Direction::Down => {
                    if row + 1 < grid.len() {
                        match grid[row + 1][col] {
                            '.' => row += 1,                  // Move down
                            _ => current_direction = Direction::Left, // change direction
                        }
                    } else {
                        // Exit from the grid
                        break;
                    }
                }
                Direction::Left => {
                    if col.checked_sub(1).is_some() {
                        match grid[row][col - 1] {
                            '.' => col -= 1,                // Move up
                            _ => current_direction = Direction::Up, // Change direction
                        }
                    } else {
                        // This is the exit from the grid means we are out
                        break;
                    }
                }
            }
        }

        Ok(visited_cells.len() as i32)
    }

    pub fn part_2(_input: &str) -> Result<i32> {
        Ok(0)
    }
}

fn guardian_start_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
    // We will have always a guardian so it's safe to start at (0,0) it will be changed later on
    let mut start_cell: Cell = (0, 0);

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '^' {
                start_cell = (row, col);
                break;
            }
        }
    }

    start_cell
}

fn load_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(Solution::part_1(input)?, 41);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#""#;
        assert_eq!(Solution::part_2(input)?, 0);

        Ok(())
    }
}

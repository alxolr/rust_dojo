use std::collections::VecDeque;

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<i32> {
        // Load the grid from the input string
        let grid = load_grid(input);

        let mut count = 0;

        // Count occurrences of "XMAS" and "SAMX" in each row
        for line in grid.iter().map(|line| line.iter().collect::<String>()) {
            count += count_xmas(&line)?;
        }

        let cols = grid[0].len();
        let rows = grid.len();

        // Count occurrences of "XMAS" and "SAMX" in each column
        for col in 0..cols {
            let line: String = (0..rows).map(|row| grid[row][col]).collect();
            let count_line = count_xmas(&line)?;
            count += count_line;
        }

        // Count occurrences in diagonals (bottom-up)
        for row in 0..rows {
            let line: String = (0..=row).map(|i| grid[row - i][i]).collect();
            let count_line: usize = count_xmas(&line)?;
            count += count_line;
        }

        for row in 1..rows {
            let line: String = (0..(rows - row))
                .map(|i| grid[row + i][cols - 1 - i])
                .collect();
            let count_line = count_xmas(&line)?;
            count += count_line;
        }

        // Count occurrences in diagonals (top-down)
        for row in 0..rows {
            let line: String = (0..=row).map(|i| grid[row - i][cols - 1 - i]).collect();
            let count_line = count_xmas(&line)?;
            count += count_line;
        }

        for row in 1..rows {
            let line: String = (0..(rows - row)).map(|i| grid[row + i][i]).collect();
            let count_line = count_xmas(&line)?;
            count += count_line;
        }

        Ok(count as i32)
    }

    pub fn part_2(input: &str) -> Result<i32> {
        let mut count = 0;
        let grid = load_grid(input);
        let solutions = ["SAM", "MAS"];

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == 'A' {
                    let row = row as isize;
                    let col = col as isize;

                    let left: String = [
                        safe_getter(&grid, row - 1, col - 1),
                        safe_getter(&grid, row, col),
                        safe_getter(&grid, row + 1, col + 1),
                    ]
                    .iter()
                    .flatten()
                    .collect();

                    let right: String = [
                        safe_getter(&grid, row - 1, col + 1),
                        safe_getter(&grid, row, col),
                        safe_getter(&grid, row + 1, col - 1),
                    ]
                    .iter()
                    .flatten()
                    .collect();

                    if solutions.contains(&left.as_str()) && solutions.contains(&right.as_str()) {
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }
}

fn safe_getter(grid: &[Vec<char>], row: isize, col: isize) -> Option<char> {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    if row >= 0 && row < rows && col >= 0 && col < cols {
        Some(grid[row as usize][col as usize])
    } else {
        None
    }
}

fn count_xmas(line: &str) -> Result<usize> {
    let mut count_occurances = 0;

    // The idea is to store the last 4 characters in a deque and check if this is one of our targets
    // "XMAS" or "SAMX"
    let mut char_queue = VecDeque::with_capacity(4);

    for ch in line.chars() {
        char_queue.push_back(ch);
        if char_queue.len() == 4 {
            let substring: String = char_queue.iter().collect();
            if substring == "XMAS" || substring == "SAMX" {
                count_occurances += 1;
            }
            char_queue.pop_front();
        }
    }

    Ok(count_occurances)
}

fn load_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_xmas() -> Result<()> {
        let input = "XMASAMXAMM";
        assert_eq!(count_xmas(input)?, 2);

        let input = "XMASAMXAMM";
        assert_eq!(count_xmas(input)?, 2);

        let input = "MAMXMASAMX";
        assert_eq!(count_xmas(input)?, 2);

        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(Solution::part_1(input)?, 18);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(Solution::part_2(input)?, 9);

        Ok(())
    }
}

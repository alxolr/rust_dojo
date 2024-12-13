use std::collections::HashSet;

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<u32> {
        let grid = load_grid(input);

        let mut visited_plants = HashSet::new();
        let mut total_cost = 0;

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if !visited_plants.contains(&(row, col)) {
                    let (area, perimiter) = compute_area_perimeter(
                        row as isize,
                        col as isize,
                        grid[row][col],
                        &grid,
                        &mut visited_plants,
                    );

                    total_cost += area * perimiter;
                }
            }
        }

        Ok(total_cost)
    }

    pub fn part_2(_input: &str) -> Result<i32> {
        Ok(0)
    }
}

fn compute_area_perimeter(
    row: isize,
    col: isize,
    requested_type: char,
    grid: &[Vec<char>],
    visited_plants: &mut HashSet<(usize, usize)>,
) -> (u32, u32) {
    match safe_get(row, col, grid) {
        Some(plant) => {
            if plant != requested_type {
                (0, 0)
            } else {
                if visited_plants.contains(&(row as usize, col as usize)) {
                    return (0, 0);
                }

                visited_plants.insert((row as usize, col as usize));
                let mut area = 1;
                let mut perimeter = count_perimeter(row, col, grid);

                let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                for (delta_row, delta_col) in directions {
                    let (z_area, z_perimeter) = compute_area_perimeter(
                        row + delta_row,
                        col + delta_col,
                        requested_type,
                        grid,
                        visited_plants,
                    );

                    area += z_area;
                    perimeter += z_perimeter;
                }

                (area, perimeter)
            }
        }
        None => (0, 0),
    }
}

fn count_perimeter(row: isize, col: isize, grid: &[Vec<char>]) -> u32 {
    let flower_type = grid[row as usize][col as usize];
    let mut perimeter = 0;
    let neighbours = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for (delta_row, delta_col) in neighbours {
        match safe_get(row as isize + delta_row, col as isize + delta_col, grid) {
            Some(x) => {
                if x != flower_type {
                    perimeter += 1;
                }
            }
            None => perimeter += 1,
        }
    }

    perimeter
}

fn safe_get(row: isize, col: isize, grid: &[Vec<char>]) -> Option<char> {
    if row >= 0 && row < grid.len() as isize && col >= 0 && col < grid[0].len() as isize {
        Some(grid[row as usize][col as usize])
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
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF  
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        assert_eq!(Solution::part_1(input)?, 1930);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#""#;
        assert_eq!(Solution::part_2(input)?, 0);

        Ok(())
    }
}

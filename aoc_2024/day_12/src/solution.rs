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

    pub fn part_2(input: &str) -> Result<u32> {
        let grid = load_grid(input);

        let mut visited_plants = HashSet::new();
        let mut total_cost = 0;

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if !visited_plants.contains(&(row, col)) {
                    let (area, perimiter) = compute_area_perimeter_part_2(
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

fn compute_area_perimeter_part_2(
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
                let mut perimeter = count_perimeter_part_2(row, col, grid);

                let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                for (delta_row, delta_col) in directions {
                    let (z_area, z_perimeter) = compute_area_perimeter_part_2(
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

fn count_perimeter_part_2(row: isize, col: isize, grid: &[Vec<char>]) -> u32 {
    // The main line directions will be top down, and left to right
    // This means we will add the line to the perimeter when the bottom is bounded or the utmoust right is bounded
    // There are a couple of cases we need to check the bounds
    // If it's bound on the right we check if the down one is different type
    // if it's not different than we have a closing line
    // if it's the same we need to check if it's not a corner
    let selected_flower = grid[row as usize][col as usize];
    let mut perimeter = 0;

    let down = (1, 0);
    let up = (-1, 0);
    let left: (isize, isize) = (0, -1);
    let right: (isize, isize) = (0, 1);
    let down_left = (1, -1);
    let down_right = (1, 1);
    let up_right = (-1, 1);

    // Checking left bound
    match safe_get(row + left.0, col + left.1, grid) {
        Some(flower) => {
            if flower != selected_flower {
                // this means it's bounded by a different flower
                // now we need to check down and diagonaly left
                if let Some(flower) = safe_get(row + down.0, col + down.1, grid) {
                    if flower != selected_flower {
                        perimeter += 1;
                    } else {
                        // check diagonaly left
                        if let Some(diagonal_flower) =
                            safe_get(row + down_left.0, col + down_left.1, grid)
                        {
                            if diagonal_flower == selected_flower {
                                // this means it's left corner witch finishes the line top down
                                perimeter += 1;
                            }
                        }
                    }
                } else {
                    perimeter += 1;
                }
            }
        }
        None => {
            // this means it's bounded on the left we check if down is not same type
            if let Some(flower) = safe_get(row + down.0, col + down.1, grid) {
                if flower != selected_flower {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    // Checking the right bound
    match safe_get(row + right.0, col + right.1, grid) {
        Some(flower_type) => {
            if flower_type != selected_flower {
                match safe_get(row + down.0, col + down.1, grid) {
                    Some(flower_type) => {
                        if flower_type != selected_flower {
                            perimeter += 1;
                        } else {
                            if let Some(diagonaly_right_flower) =
                                safe_get(row + down_right.0, col + down_right.1, grid)
                            {
                                if diagonaly_right_flower == selected_flower {
                                    perimeter += 1;
                                } // it's a diagonally right corner
                            }
                        }
                    }
                    None => {
                        perimeter += 1;
                    }
                }
            }
        }
        None => {
            // we check down if it's different type than it's the end of line +1
            match safe_get(row + down.0, col + down.1, grid) {
                Some(flower_type) => {
                    if flower_type != selected_flower {
                        perimeter += 1;
                    }
                }
                None => {
                    perimeter += 1;
                }
            }
        }
    }

    // Checking the up bound
    match safe_get(row + up.0, col + up.1, grid) {
        Some(up_flower) => {
            if up_flower != selected_flower {
                // check right
                match safe_get(row + right.0, col + right.1, grid) {
                    Some(right_flower) => {
                        if right_flower != selected_flower {
                            perimeter += 1;
                        } else {
                            if let Some(up_right_flower) =
                                safe_get(row + up_right.0, col + up_right.1, grid)
                            {
                                if up_right_flower == selected_flower {
                                    perimeter += 1;
                                }
                            }
                        }
                    }
                    None => {
                        perimeter += 1;
                    }
                }
            }
        }
        None => {
            // check right
            match safe_get(row + right.0, col + right.1, grid) {
                Some(right_flower) => {
                    if right_flower != selected_flower {
                        perimeter += 1;
                    }
                }
                None => {
                    perimeter += 1;
                }
            }
        }
    }

    match safe_get(row + down.0, col + down.1, grid) {
        Some(down_flower) => {
            if down_flower != selected_flower {
                match safe_get(row + right.0, col + right.1, grid) {
                    Some(right_flower) => {
                        if right_flower != selected_flower {
                            perimeter += 1;
                        } else {
                            if let Some(down_right_flower) =
                                safe_get(row + down_right.0, col + down_right.1, grid)
                            {
                                if down_right_flower == selected_flower {
                                    perimeter += 1;
                                }
                            }
                        }
                    }
                    None => {
                        perimeter += 1;
                    }
                }
            }
        }
        None => {
            // bound bottom check right
            match safe_get(row + right.0, col + right.1, grid) {
                Some(right_flower) => {
                    if right_flower != selected_flower {
                        perimeter += 1;
                    }
                }
                None => {
                    perimeter += 1;
                }
            }
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
        assert_eq!(Solution::part_2(input)?, 1206);

        let input = r#"RRRRI
RRRRI
VRRRC
VVRCC"#;

        assert_eq!(Solution::part_2(input)?, 164);

        let input = r#"CCX
XCC"#;
        assert_eq!(Solution::part_2(input)?, 40);

        Ok(())
    }
}

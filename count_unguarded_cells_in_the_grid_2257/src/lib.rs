pub struct Solution;

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Visible = 1,
    Invisible = 0,
    Guard = 2,
    Wall = 3,
}

impl Solution {
    /// We have matrix of cells mxn and we do have walls and guards, we need to find out all unguarded cells
    /// Algorithm 1
    /// Generate the matrix mxn all with zeros this will be the cells
    /// Fill in the walls will be -1
    /// Fill in the guards -2
    /// For each guard expand it's vision up down left and right until they hit a wall mark all -3
    /// In the end count all invisible cells which has the number 0
    ///
    /// Analysis
    /// Complexity above algorithm is O(g*n*m) which is cubic
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let cols = n as usize;
        let rows = m as usize;

        // Generate the matrix all cell being Invisible at first
        let mut matrix = vec![vec![Cell::Invisible; cols]; rows];

        // Place the guards and the walls
        for guard in guards.iter() {
            let [row, col] = guard[..] else {
                panic!("Invalid input")
            };
            matrix[row as usize][col as usize] = Cell::Guard;
        }
        for wall in walls.iter() {
            let [row, col] = wall[..] else {
                panic!("Invalid input")
            };
            matrix[row as usize][col as usize] = Cell::Wall;
        }

        // go vertical cell by cell

        for row in 0..rows {
            // go from left to right we set the line of sight disabled at first
            // it will be enabled at the moment there is a guard
            let mut is_in_line_of_sight = false;
            for col in 0..cols {
                apply_line_of_sight(&mut matrix, row, col, &mut is_in_line_of_sight);
            }

            // go indirect
            let mut is_in_line_of_sight = false;
            for col in (0..cols).rev() {
                apply_line_of_sight(&mut matrix, row, col, &mut is_in_line_of_sight);
            }
        }

        // go vertically
        for col in 0..cols {
            // go left to right
            let mut is_in_line_of_sight = false;
            for row in 0..rows {
                apply_line_of_sight(&mut matrix, row, col, &mut is_in_line_of_sight);
            }

            // go right to left
            let mut is_in_line_of_sight = false;
            for row in (0..rows).rev() {
                apply_line_of_sight(&mut matrix, row, col, &mut is_in_line_of_sight);
            }
        }

        // Count the Remaining Invisible cells
        // Flatten the cells and filter the invisible ones
        matrix
            .iter()
            .flatten()
            .filter(|x| **x == Cell::Invisible)
            .count() as i32
    }
}

fn apply_line_of_sight(
    matrix: &mut Vec<Vec<Cell>>,
    row: usize,
    col: usize,
    is_in_line_of_sight: &mut bool,
) {
    match matrix[row][col] {
        Cell::Guard => {
            *is_in_line_of_sight = true;
        }
        Cell::Wall => {
            *is_in_line_of_sight = false;
        }
        Cell::Invisible => {
            if *is_in_line_of_sight {
                matrix[row][col] = Cell::Visible;
            }
        }
        _ => {} // continue
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (
                    4,
                    6,
                    vec![vec![0, 0], vec![1, 1], vec![2, 3]],
                    vec![vec![0, 1], vec![2, 2], vec![1, 4]],
                ),
                7,
            ),
            (
                (
                    3,
                    3,
                    vec![vec![1, 1]],
                    vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]],
                ),
                4,
            ),
            (
                (
                    2,
                    7,
                    vec![vec![1, 5], vec![1, 1], vec![1, 6], vec![0, 2]],
                    vec![vec![0, 6], vec![0, 3], vec![0, 5]],
                ),
                1,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((m, n, guards, walls), expected))| {
                let result = Solution::count_unguarded(m, n, guards, walls);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut grid = grid;
        // gather the diagonals
        let mut diagonals: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                let diagonal_key = row as i32 - col as i32;
                diagonals
                    .entry(diagonal_key)
                    .or_insert(Vec::new())
                    .push(grid[row][col]);
            }
        }

        // Sort each diagonal: positive keys descending, negative keys ascending
        for (key, values) in diagonals.iter_mut() {
            if *key >= 0 {
                values.sort_by(|a, b| b.cmp(a)); // descending
            } else {
                values.sort_by(|a, b| a.cmp(b)); // ascending
            }
        }

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                let key = row as i32 - col as i32;
                grid[row][col] = diagonals.get_mut(&key).unwrap().remove(0);
            }
        }

        grid
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(
            vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]],
            vec![vec![8, 2, 3], vec![9, 6, 7], vec![4, 5, 1]],
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::sort_matrix(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

pub struct Solution;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let flattened_grid = grid.into_iter().flatten().collect::<Vec<_>>();
        let total_elements = flattened_grid.len();

        if total_elements == 1 {
            return 1;
        }

        let remainder_check = flattened_grid[0] % x;

        if flattened_grid
            .iter()
            .any(|&value| value % x != remainder_check)
        {
            return -1;
        }

        // Sort the flattened grid
        let mut sorted_grid = flattened_grid;
        sorted_grid.sort_unstable();

        // let average_value = sorted_grid.iter().sum::<i32>() / total_elements as i32;

        // Find the median value
        let mean_value = if total_elements % 2 == 0 {
            (sorted_grid[total_elements / 2] + sorted_grid[total_elements / 2 - 1]) / 2
        } else {
            sorted_grid[total_elements / 2]
        };

        // Find the closest number to the average
        let closest_element = sorted_grid
            .iter()
            .enumerate()
            .map(|(index, &value)| (index, (value - mean_value).abs()))
            .min_by(|a: &(usize, i32), b| a.1.cmp(&b.1));

        match closest_element {
            Some((closest_index, _)) => {
                let target_value = sorted_grid[closest_index];

                sorted_grid
                    .iter()
                    .map(|&value| (value - target_value).abs() / x)
                    .sum()
            }
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![vec![2, 4], vec![6, 8]], 2), 4),
            ((vec![vec![1, 5], vec![2, 3]], 1), 5),
            ((vec![vec![1, 2], vec![3, 4]], 2), -1),
            (
                (
                    vec![
                        vec![980, 476, 644, 56],
                        vec![644, 140, 812, 308],
                        vec![812, 812, 896, 560],
                        vec![728, 476, 56, 812],
                    ],
                    84,
                ),
                45,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((grid, x), expected))| {
                let result = Solution::min_operations(grid, x);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

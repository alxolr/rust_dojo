const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut cache = vec![vec![vec![0; k as usize]; cols + 1]; rows + 1];
        let target_remainder = (k - grid[rows - 1][cols - 1] % k) % k;

        cache[rows - 1][cols - 1][target_remainder as usize] = 1;

        for row in (0..rows).rev() {
            for col in (0..cols).rev() {
                if row == rows - 1 && col == cols - 1 {
                    continue;
                }
                for remainder in 0..k {
                    let new_remainder = (remainder + grid[row][col]) % k;
                    cache[row][col][remainder as usize] =
                        (cache[row + 1][col][new_remainder as usize]) % MOD
                            + (cache[row][col + 1][new_remainder as usize]) % MOD;
                }
            }
        }

        cache[0][0][0]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![((vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]], 3), 2)];

        for (idx, ((grid, k), expected)) in scenarios.into_iter().enumerate() {
            println!("Scenario {}", idx + 1);
            assert_eq!(Solution::number_of_paths(grid, k), expected);
        }
    }
}

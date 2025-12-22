impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        // After doing some brainstorming with Satan looks like the way to go is to find the longest
        // increasing substring using a dp aproach
        let cols = strs[0].len();
        let grid = strs
            .into_iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // dp[i] = maximum number of columns we can keep where column i is the last column
        let mut dp = vec![1; cols];

        // For each column i, check all previous columns j to see if we can extend the sequence
        for i in 0..cols {
            for j in 0..i {
                // Check if column j can come before column i (all rows satisfy grid[row][j] <= grid[row][i])
                if is_solution(&get_column(j, &grid), &get_column(i, &grid)) {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }

        // Maximum columns we can keep
        let max_kept = *dp.iter().max().unwrap_or(&1);

        (cols - max_kept) as i32
    }
}

fn get_column(col: usize, grid: &Vec<Vec<char>>) -> Vec<&char> {
    grid.iter().flat_map(|line| line.get(col)).collect()
}

fn is_solution(left: &Vec<&char>, right: &Vec<&char>) -> bool {
    left.iter().zip(right.iter()).all(|(l, r)| l <= r)
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec!["babca".to_string(), "bbazb".to_string()], 3),
            (vec!["babbba".to_string(), "bbaaab".to_string()], 3),
            (
                vec![
                    "aabbaa".to_string(),
                    "baabab".to_string(),
                    "aaabab".to_string(),
                ],
                3,
            ),
            (vec!["edcba".to_string()], 4),
            (
                vec!["ghi".to_string(), "def".to_string(), "abc".to_string()],
                0,
            ),
        ];

        for (idx, (input, expected)) in scenarios.into_iter().enumerate() {
            let result = Solution::min_deletion_size(input.clone());
            println!(
                "scenario {} - expected: {}, got: {}",
                idx + 1,
                expected,
                result
            );
            assert_eq!(result, expected);
            println!("scenario {} ok", idx + 1);
        }
    }
}

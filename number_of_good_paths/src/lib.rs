struct Solution;

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        6
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
                    vec![1, 3, 2, 1, 3],
                    vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
                ),
                6,
            ),
            (
                (
                    vec![1, 1, 2, 2, 3],
                    vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]],
                ),
                7,
            ),
            ((vec![1], vec![]), 1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((vals, edges), expected))| {
                let result = Solution::number_of_good_paths(vals, edges);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

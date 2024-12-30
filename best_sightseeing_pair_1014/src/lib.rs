pub struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        sight_seeing(0, 1, values)
    }
}

fn sight_seeing(i: usize, j: usize, values: &[i32]) -> i32 {
    if i >= values.len() {
        return 0;
    }

    let score = values[i] + values[j] + (i - j) as i32;

    (i + 1..values.len())
        .map(|j| sight_seeing(i, j, values))
        .chain([score].into_iter())
        .max()
        .unwrap_or(score)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![8, 1, 5, 2, 6], 11)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_score_sightseeing_pair(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

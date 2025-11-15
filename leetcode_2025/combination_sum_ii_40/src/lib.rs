impl Solution {
    fn backtrack(
        acc: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>,
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
    ) {
        let sum: i32 = acc.iter().sum();

        if sum == target {
            results.push(acc.clone());
            return;
        } else if sum > target {
            return;
        }

        for i in start..candidates.len() {
            if i > start && candidates[i] == candidates[i - 1] {
                continue; // skip duplicates
            }
            acc.push(candidates[i]);
            Self::backtrack(acc, results, candidates, target, i + 1); // i, not i+1, allows reuse
            acc.pop();
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();
        let mut candidates = candidates;
        candidates.sort_unstable();

        Self::backtrack(&mut Vec::new(), &mut results, &candidates, target, 0);
        
        results
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(
            (vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::combination_sum(input.0, input.1);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

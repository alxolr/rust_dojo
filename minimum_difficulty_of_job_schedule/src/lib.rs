use std::{cmp::min, collections::HashMap};

struct Solution;

fn dfs(
    memo: &mut HashMap<(usize, i32, i32), i32>,
    idx: usize,
    days: i32,
    mut cur_max: i32,
    job_difficulties: &[i32],
) -> i32 {
    if let Some(result) = memo.get(&(idx, days, cur_max)) {
        return *result;
    }

    if job_difficulties.len() < days as usize {
        return -1;
    }

    if idx == job_difficulties.len() {
        if days == 0 {
            return 0;
        } else {
            return i32::MAX / 2;
        }
    }

    if days == 0 {
        return i32::MAX / 2;
    }

    cur_max = cur_max.max(job_difficulties[idx]);

    let result = min(
        dfs(memo, idx + 1, days, cur_max, job_difficulties),
        cur_max + dfs(memo, idx + 1, days - 1, -1, job_difficulties),
    );

    memo.insert((idx, days, cur_max), result);

    result
}

impl Solution {
    pub fn min_difficulty(job_difficulties: Vec<i32>, d: i32) -> i32 {
        let mut memo = HashMap::new();

        dfs(&mut memo, 0, d, -1, &job_difficulties)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mimum_difficulty_ok() {
        let scenarios = vec![
            ((vec![6, 5, 4, 3, 2, 1], 2), 7),
            ((vec![9, 9, 9], 4), -1),
            ((vec![1, 1, 1], 3), 3),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((jobs, days), expected))| {
                let result = Solution::min_difficulty(jobs, days);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

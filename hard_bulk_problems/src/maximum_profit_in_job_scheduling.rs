use std::collections::HashMap;

pub struct Solution;

fn dfs(memo: &mut HashMap<usize, i32>, i: usize, jobs: &Vec<(i32, i32, i32)>) -> i32 {
    if let Some(result) = memo.get(&i) {
        return *result;
    }

    if i == jobs.len() {
        return 0;
    }

    let result = dfs(memo, i + 1, jobs);

    let mut j = i + 1;

    while j < jobs.len() {
        if jobs[i].1 <= jobs[j].0 {
            break;
        } else {
            j += 1;
        }
    }

    let result = result.max(jobs[i].2 + dfs(memo, j, jobs));

    memo.insert(i, result);

    result
}

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = start_time
            .into_iter()
            .zip(end_time)
            .zip(profit)
            .map(|((s, e), p)| (s, e, p))
            .collect::<Vec<_>>();

        jobs.sort_by(|a, b| a.0.cmp(&b.0));

        dfs(&mut HashMap::new(), 0, &jobs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (
                (vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
                120,
            ),
            (
                (
                    vec![1, 2, 3, 4, 6],
                    vec![3, 5, 10, 6, 9],
                    vec![20, 20, 100, 70, 60],
                ),
                150,
            ),
            ((vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]), 6),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((starts, ends, profits), expected))| {
                let result = Solution::job_scheduling(starts, ends, profits);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

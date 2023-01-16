struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        match intervals.len() {
            0 => vec![new_interval],
            _ => {
                let mut intervals = intervals.clone();

                if let Some(idx) = intervals.iter().position(|x| new_interval[0] < x[0]) {
                    intervals.insert(idx, new_interval)
                } else {
                    intervals.insert(intervals.len(), new_interval)
                }

                let mut stack = vec![intervals[0].clone()];

                for idx in 1..intervals.len() {
                    let mut top = stack.last().unwrap().clone();

                    if top[1] < intervals[idx][0] {
                        stack.push(intervals[idx].clone());
                    } else if top[1] < intervals[idx][1] {
                        top[1] = intervals[idx][1];
                        stack.pop();
                        stack.push(top);
                    }
                }

                stack.into_iter().collect::<Vec<_>>()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![vec![1, 5]], vec![2, 7]), vec![vec![1, 7]]),
            (
                (vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
                vec![vec![1, 5], vec![6, 9]],
            ),
            ((vec![], vec![4, 7]), vec![vec![4, 7]]),
            (
                (
                    vec![
                        vec![1, 2],
                        vec![3, 5],
                        vec![6, 7],
                        vec![8, 10],
                        vec![12, 16],
                    ],
                    vec![4, 8],
                ),
                vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            ),
        ];

        scenarios.into_iter().enumerate().for_each(
            |(idx, ((intervals, new_interval), expected))| {
                let result = Solution::insert(intervals, new_interval);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            },
        );
    }
}

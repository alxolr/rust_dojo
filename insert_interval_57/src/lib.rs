struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut new_interval = new_interval;

        for interval in intervals {
            if !Self::overlap(&interval, &new_interval) {
                result.push(interval);
            } else {
                new_interval = Self::intersect(&interval, new_interval).unwrap();
            }
        }

        result.push(new_interval);
        result.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        result
            .into_iter()
            .fold(vec![], |mut acc, interval| {
                if let Some(last) = acc.last_mut() {
                    if Self::overlap(last, &interval) {
                        *last = Self::intersect(last, interval).unwrap();
                    } else {
                        acc.push(interval);
                    }
                } else {
                    acc.push(interval);
                }

                acc
            })



    }

    fn overlap(a: &Vec<i32>, b: &Vec<i32>) -> bool {
        let a_start = a[0];
        let a_end = a[1];
        let b_start = b[0];
        let b_end = b[1];

        a_start <= b_end && b_start <= a_end
    }

    fn intersect(a: &Vec<i32>, b: Vec<i32>) -> Option<Vec<i32>> {
        let a_start = a[0];
        let a_end = a[1];
        let b_start = b[0];
        let b_end = b[1];

        if b_start > a_end || a_start > b_end {
            None
        } else {
            let o_start = i32::max(a_start, b_start);
            let o_end = i32::max(a_end, b_end);

            Some(vec![o_start, o_end])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
                vec![vec![1, 5], vec![6, 9]],
            ),
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

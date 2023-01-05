pub struct Solution;

struct Interval(i32, i32);

impl Interval {
    pub fn intersect(&self, other: &Interval) -> Option<Interval> {
        if other.0 > self.1 || self.0 > other.1 {
            None
        } else {
            let start = self.0.max(other.0);
            let end = self.1.min(other.1);

            Some(Interval(start, end))
        }
    }
}

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut intervals = points
            .iter()
            .map(|item| Interval(item[0], item[1]))
            .collect::<Vec<_>>();

        if intervals.len() == 1 {
            return 1;
        }

        intervals.sort_by(|x, y| {
            if x.0 == y.0 {
                x.1.cmp(&y.1)
            } else {
                x.0.cmp(&y.0)
            }
        });

        let intersections =
            intervals
                .into_iter()
                .fold(vec![], |mut acc: Vec<Interval>, item: Interval| {
                    if acc.is_empty() {
                        acc.push(item)
                    } else {
                        // find one with intersection
                        let maybe_intersect = acc
                            .iter()
                            .enumerate()
                            .find(|(_, value)| item.intersect(value).is_some());

                        if let Some((idx, found)) = maybe_intersect {
                            let intersected_interval = found.intersect(&item).unwrap();

                            acc[idx] = intersected_interval;
                        } else {
                            acc.push(item);
                        }
                    }

                    acc
                });

        intersections.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenarios() {
        let scenarios = vec![
            (vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]], 2),
            (vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]], 4),
            (vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]], 2),
            (
                vec![
                    vec![3, 9],
                    vec![7, 12],
                    vec![3, 8],
                    vec![6, 8],
                    vec![9, 10],
                    vec![2, 9],
                    vec![0, 9],
                    vec![3, 9],
                    vec![0, 6],
                    vec![2, 8],
                ],
                2,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::find_min_arrow_shots(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

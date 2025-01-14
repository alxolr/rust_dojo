pub struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        a.iter()
            .zip(b.iter())
            .scan(
                (HashSet::new(), HashSet::new()),
                |(a_set, b_set), (a, b)| {
                    a_set.insert(a);
                    b_set.insert(b);
                    let count = a_set.intersection(&b_set).count();

                    Some(count as i32)
                },
            )
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![((vec![1, 3, 2, 4], vec![3, 1, 2, 4]), vec![0, 2, 3, 4])];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((a, b), expected))| {
                let result = Solution::find_the_prefix_common_array(a, b);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

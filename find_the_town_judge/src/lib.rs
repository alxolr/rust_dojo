pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let relations = trust.iter().fold(vec![0; n as usize + 1], |mut acc, item| {
            acc[item[0] as usize] -= 1;
            acc[item[1] as usize] += 1;

            acc
        });

        relations
            .iter()
            .enumerate()
            .find_map(|(idx, val)| {
                if *val == n - 1 {
                    Some(idx as i32)
                } else {
                    None
                }
            })
            .unwrap_or(-1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((2, vec![vec![1, 2]]), 2),
            ((3, vec![vec![1, 3], vec![2, 3]]), 3),
            ((3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]), -1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((n, trust), expected))| {
                let result = Solution::find_judge(n, trust);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if trust.len() == 0 && n == 1 {
            return 1;
        }

        let mut count = vec![0; n as usize + 1];
        for person in trust.into_iter() {
            count[person[0] as usize] -= 1;
            count[person[1] as usize] += 1;
        }

        for person in 0..count.len() {
            if count[person] == n - 1 {
                return person as i32;
            }
        }

        -1
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

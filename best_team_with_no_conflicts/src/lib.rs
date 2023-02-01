pub struct Solution;

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {

        let mut idx: Vec<usize> = (0..scores.len()).collect();

        idx.sort_unstable_by(|&a, &b| if ages[a] == ages[b] {
            scores[a].cmp(&scores[b])
        } else {
            ages[a].cmp(&ages[b])
        });

        let mut dp = vec![0; scores.len()];
        let mut ans = -1;
        
        for i in 0..scores.len() {
            let t = scores[idx[i]];
            dp[i] = t;
            for j in 0..i {
                if scores[idx[j]] <= t {
                    dp[i] = dp[i].max(dp[j] + t);
                }
            }
            ans = ans.max(dp[i]);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![1, 3, 5, 10, 15], vec![1, 2, 3, 4, 5]), 34),
            ((vec![4, 5, 6, 5], vec![2, 1, 2, 1]), 16),
            ((vec![1, 2, 3, 5], vec![8, 9, 10, 1]), 6),
            (
                (
                    vec![1, 3, 7, 3, 2, 4, 10, 7, 5],
                    vec![4, 5, 2, 1, 1, 2, 4, 1, 4],
                ),
                29,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((scores, ages), expected))| {
                let result = Solution::best_team_score(scores, ages);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

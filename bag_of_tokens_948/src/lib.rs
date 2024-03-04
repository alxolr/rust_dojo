struct Solution;
impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        let mut tokens = tokens;
        let mut power = power;

        if tokens.is_empty() {
            return 0;
        }

        tokens.sort();

        let mut max_score = 0;
        let mut cur_score = 0;

        let mut left = 0;
        let mut right = tokens.len() - 1;

        while left <= right {
            if power - tokens[left] >= 0 {
                power -= tokens[left];
                cur_score += 1;
                max_score = max_score.max(cur_score);
                left += 1;
            } else if cur_score > 0 {
                power += tokens[right];
                cur_score -= 1;
                if right == 0 {
                    break;
                }
                right -= 1;
            } else {
                break;
            }
        }

        max_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![100], 50), 0),
            ((vec![200, 100], 150), 1),
            ((vec![], 85), 0),
            ((vec![71, 55, 82], 54), 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((tokens, power), expected))| {
                let result = Solution::bag_of_tokens_score(tokens, power);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

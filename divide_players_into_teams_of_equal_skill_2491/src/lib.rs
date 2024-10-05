pub struct Solution;

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let players = skill.len();
        if players % 2 != 0 {
            return -1;
        }

        let mut skill = skill;
        skill.sort_unstable();

        let skill_sum = skill[0] + skill[players - 1];
        let mut total_score = 0;

        for i in 0..players / 2 {
            if skill[i] + skill[players - 1 - i] != skill_sum {
                return -1;
            }
            total_score += (skill[i] * skill[players - 1 - i]) as i64;
        }

        total_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![3, 2, 5, 1, 3, 4], 22)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::divide_players(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

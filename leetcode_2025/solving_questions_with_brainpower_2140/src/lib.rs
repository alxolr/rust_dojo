use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn dp(memo: &mut HashMap<usize, i64>, id: usize, questions: &Vec<(i64, usize)>) -> i64 {
        if let Some(result) = memo.get(&id) {
            return *result;
        }

        if id >= questions.len() {
            return 0;
        }

        let (points, brainpower) = questions[id];

        let result = i64::max(
            points + Solution::dp(memo, id + brainpower + 1, questions),
            Solution::dp(memo, id + 1, questions),
        );

        memo.insert(id, result);

        result
    }

    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let questions = questions
            .into_iter()
            .map(|item| (item[0] as i64, item[1] as usize))
            .collect::<Vec<_>>();

        Solution::dp(&mut HashMap::new(), 0, &questions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]], 5),
            (
                vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]],
                7,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::most_points(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

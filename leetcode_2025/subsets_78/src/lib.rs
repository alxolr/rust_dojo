impl Solution {
    fn backtrack(i: usize, acc: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, values: &Vec<i32>) {
        result.push(acc.clone());

        for idx in i..values.len() {
            acc.push(values[idx]);
            Self::backtrack(idx + 1, acc, result, values);
            acc.pop();
        }
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = vec![];

        Self::backtrack(0, &mut vec![], &mut results, &nums);

        results
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(
            vec![1, 2, 3],
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ],
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::subsets(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

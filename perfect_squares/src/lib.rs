use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut memo = HashMap::new();
        let perfect_squares = Self::gen_perfect_squares(n);

        Self::dfs(&mut memo, n, &perfect_squares)
    }

    fn gen_perfect_squares(n: i32) -> Vec<i32> {
        (1..=n).map(|x| x * x).take_while(|x| x <= &n).collect()
    }

    fn dfs(memo: &mut HashMap<i32, i32>, n: i32, perfect_squares: &Vec<i32>) -> i32 {
        if let Some(result) = memo.get(&n) {
            return *result;
        }

        if n == 0 {
            return 0;
        }

        let mut result = i32::MAX;
        for square in perfect_squares {
            if *square <= n {
                result = result.min(1 + Self::dfs(memo, n - *square, perfect_squares));
            }
        }

        memo.insert(n, result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(12, 3), (13, 2)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::num_squares(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

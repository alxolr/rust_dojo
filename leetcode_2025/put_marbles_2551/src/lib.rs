pub struct Solution;

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let n = weights.len();
        let mut boundaries = weights
            .windows(2)
            .into_iter()
            .map(|x| x[0] + x[1])
            .collect::<Vec<_>>();

        boundaries.sort();

        let mut answer = 0;

        for i in 0..k as usize - 1 {
            answer += (boundaries[n - 2 - i] - boundaries[i]) as i64
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![((vec![1, 3, 5, 1], 2), 4), ((vec![1, 3], 2), 0)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((weights, k), expected))| {
                let result = Solution::put_marbles(weights, k);

                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

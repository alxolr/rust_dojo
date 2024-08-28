pub struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        fn dp(is_alice: bool, idx: usize, m: usize, piles: &Vec<i32>) -> i32 {
            if idx == piles.len() {
                return 0;
            }

            let mut res = 0;

            let mut total = 0;
            for x in 1..=2 * m {
                if idx + x > piles.len() {
                    break;
                }

                total += piles[idx + x - 1];

                if is_alice {
                    res = i32::max(res, total + dp(!is_alice, idx + x, usize::max(m, x), piles));
                } else {
                    res = i32::min(res, dp(!is_alice, idx + x, m.max(x), piles));
                }
            }

            res
        }

        dp(true, 0, 1, &piles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![2, 7, 9, 4, 4], 10), (vec![1, 2, 3, 4, 5, 100], 104)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::stone_game_ii(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

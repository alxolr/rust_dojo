use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut deck = deck;
        deck.sort_unstable();
        let len = deck.len();
        let mut res = vec![0; len];

        let mut queue = VecDeque::from_iter(0..len);

        for it in deck {
            if let Some(idx) = queue.pop_front() {
                res[idx] = it;

                if let Some(next) = queue.pop_front() {
                    queue.push_back(next);
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![17, 13, 11, 2, 3, 5, 7], vec![2, 13, 3, 11, 5, 17, 7]),
            (vec![1, 1000], vec![1, 1000]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::deck_revealed_increasing(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

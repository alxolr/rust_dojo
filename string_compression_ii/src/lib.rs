#![allow(dead_code)]

use std::{cmp::min, collections::HashMap};
struct Solution;

#[derive(Clone, Eq, Hash, PartialEq)]
struct State(usize, i32, String, usize);

fn count(
    memo: &mut HashMap<State, i32>,
    i: usize,
    k: i32,
    prev: String,
    prev_cnt: usize,
    s: String,
) -> i32 {
    if let Some(result) = memo.get(&State(i, k, prev.clone(), prev_cnt)) {
        return *result;
    }

    if k < 0 {
        return i32::MAX;
    }

    if i == s.len() {
        return 0;
    }

    let curr = &s[i..=i];
    let res = if curr == prev {
        let incr = if vec![1usize, 9, 99].contains(&prev_cnt) {
            1
        } else {
            0
        };
        incr + count(memo, i + 1, k, prev.clone(), prev_cnt + 1, s.clone())
    } else {
        min(
            count(memo, i + 1, k - 1, prev.clone(), prev_cnt, s.clone()),
            1 + count(memo, i + 1, k, curr.to_string(), 1, s.clone()),
        )
    };

    memo.insert(State(i, k, prev, prev_cnt), res);

    res
}

/// This struct represents a solution to the problem of compressing a string with certain constraints.
/// The `get_length_of_optimal_compression` method calculates the length of the optimal compressed string
/// given the original string `s` and the maximum number of characters `k` that can be deleted.
impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let mut memo = HashMap::new();

        count(&mut memo, 0, k, "".to_string(), 0, s.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_length_of_optimal_compression_ok() {
        let scenarios = vec![
            (("aaabcccd".to_string(), 2), 4),
            (("aaabbaa".to_string(), 2), 2),
            (("aaaaaaaaaaa".to_string(), 0), 3),
            (("aabaabbcbbbaccc".to_string(), 6), 4),
            (("abcdefghijklmnopqrstuvwxyz".to_string(), 16), 10),
            (("ccacbaacabaabbcaccbabccacbbac".to_string(), 9), 12),
            (("prggtdthbfnnllhfhpainqpnbpajicnodhfcrpadpmbobhrjmftdrmaqqnedmjdmjdakfbgtcjfejmtaacicdkf".to_string(), 49), 31),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((s, k), expected))| {
                let result = Solution::get_length_of_optimal_compression(s, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

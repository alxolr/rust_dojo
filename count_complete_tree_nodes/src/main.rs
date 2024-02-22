use std::{cmp::Ordering, collections::HashSet};

struct Solution;

impl Solution {
    pub fn is_nice(s: &str) -> bool {
        let complement_delta = 'a' as u8 - 'A' as u8;

        let (small_letters, big_letters): (Vec<_>, Vec<u8>) =
            s.as_bytes().into_iter().partition(|x| **x >= 97);


        if small_letters.len() != big_letters.len() {
            return false;
        }

        for code in big_letters {
            if !small_letters.contains(&(code + complement_delta)) {
                return false;
            }
        }

        true
    }

    pub fn longest_nice_substring(s: String) -> String {
        let mut substrs = vec![];
        for i in 0..s.len() - 1 {
            for j in i + 1..s.len() {
                substrs.push(&s[i..=j])
            }
        }

        let solutions = substrs
            .into_iter()
            .filter(|x: &&str| Solution::is_nice(x))
            .map(|x| (x.len(), x))
            .max_by(
                |(a, _), (b, _)| {
                    if a == b {
                        Ordering::Greater
                    } else {
                        a.cmp(&b)
                    }
                },
            )
            .unwrap_or((0, ""));

        solutions.1.to_owned()
    }
}

fn main() {
    let result = Solution::longest_nice_substring("dDzeE".to_string());

    println!("{}", result);
}

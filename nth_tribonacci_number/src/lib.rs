use std::collections::HashMap;

pub struct Solution;

fn trib(memo: &mut HashMap<i32, i32>, n: i32) -> i32 {
    let maybe_found = memo.get(&n);

    if let Some(found) = maybe_found {
        *found
    } else {
        match n {
            0 => 0,
            1 => 1,
            2 => 1,
            _ => {
                let result = trib(memo, n - 1) + trib(memo, n - 2) + trib(memo, n - 3);

                memo.insert(n, result);

                result
            }
        }
    }
}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut hash_map = HashMap::new();

        trib(&mut hash_map, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(4, 4), (25, 1389537)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::tribonacci(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

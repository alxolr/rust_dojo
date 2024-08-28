use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        if n <= 1 {
            return 0;
        }

        let mut memo = HashMap::new();
        1 + Self::dp(&mut memo, 1, 1, n)
    }

    fn dp(
        memo: &mut HashMap<(i32, i32), i32>,
        current_count: i32,
        clipboard_count: i32,
        target: i32,
    ) -> i32 {
        if let Some(&result) = memo.get(&(current_count, clipboard_count)) {
            return result;
        }

        if current_count == target {
            return 0;
        }

        if current_count > target {
            return i32::MAX / 2;
        }

        let paste_steps = 1 + Self::dp(memo, current_count + clipboard_count, clipboard_count, target);
        let copy_paste_steps = 2 + Self::dp(memo, current_count + current_count, current_count, target);

        let result = i32::min(paste_steps, copy_paste_steps);
        memo.insert((current_count, clipboard_count), result);

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(3, 3)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::min_steps(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

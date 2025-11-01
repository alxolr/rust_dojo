use std::collections::HashMap;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let colors: Vec<char> = colors.chars().collect();
        let mut memo = HashMap::new();
        
        Self::solve(&colors, &needed_time, 0, None, &mut memo)
    }
    
    fn solve(
        colors: &[char],
        needed_time: &[i32],
        pos: usize,
        last_kept_color: Option<char>,
        memo: &mut HashMap<(usize, Option<char>), i32>
    ) -> i32 {
        // Base case: processed all balloons
        if pos >= colors.len() {
            return 0;
        }
        
        // Check memo
        let key = (pos, last_kept_color);
        if let Some(&cached) = memo.get(&key) {
            return cached;
        }
        
        let current_color = colors[pos];
        let mut min_cost = i32::MAX;
        
        // Option 1: Remove current balloon
        let remove_cost = needed_time[pos] + Self::solve(colors, needed_time, pos + 1, last_kept_color, memo);
        min_cost = min_cost.min(remove_cost);
        
        // Option 2: Keep current balloon (only if different from last kept color)
        if last_kept_color.is_none() || last_kept_color.unwrap() != current_color {
            let keep_cost = Self::solve(colors, needed_time, pos + 1, Some(current_color), memo);
            min_cost = min_cost.min(keep_cost);
        }
        
        memo.insert(key, min_cost);
        min_cost
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (("abaac", vec![1, 2, 3, 4, 5]), 3),
            (("abc", vec![1, 2, 3]), 0),
            (("aabaa", vec![1, 2, 3, 4, 1]), 2),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::min_cost(input.0.to_string(), input.1);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

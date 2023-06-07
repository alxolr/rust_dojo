struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap_stones = std::collections::BinaryHeap::from(stones);

        while heap_stones.len() > 1 {
            let first = heap_stones.pop().unwrap_or(0);
            let second = heap_stones.pop().unwrap_or(0);

            if first != second {
                heap_stones.push(first - second);
            }
        }

        heap_stones.pop().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![2, 7, 4, 1, 8, 1], 1)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::last_stone_weight(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn dominant_element(store: &HashMap<&i32, i32>, size: usize) -> Option<(i32, usize)> {
        store
            .iter()
            .find(|(_, frequency)| *frequency * 2 > size as i32)
            .map(|(&element, count)| (*element, *count as usize))
    }

    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let size = nums.len();
        let frequencies: HashMap<&i32, i32> = nums.iter().fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

        if let Some((dominant_element, total_count)) = Self::dominant_element(&frequencies, size) {
            let (mut left_count, mut right_count) = (0, total_count);

            for (idx, &num) in nums.iter().enumerate() {
                if num == dominant_element {
                    left_count += 1;
                    right_count -= 1;
                }

                if left_count * 2 > idx + 1 && right_count * 2 > size - (idx + 1) {
                    return idx as i32;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![1, 2, 2, 2], 2),
            (vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1], 4),
            (vec![3, 3, 3, 3, 7, 2, 2], -1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::minimum_index(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

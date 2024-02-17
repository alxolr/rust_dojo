use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut k = k;
        let frequencies = arr.iter().fold(HashMap::new(), |mut acc, item| {
            let entry = acc.entry(item).or_insert(0u32);
            *entry += 1;
            acc
        });

        let freq_frequencies =
            frequencies
                .values()
                .into_iter()
                .fold(vec![0; 10], |mut acc, item: &u32| {
                    acc[*item as usize] += 1;

                    acc
                });

        let mut clone_freq: Vec<i32> = freq_frequencies.clone();

        for (idx, value) in freq_frequencies.iter().enumerate() {
            if *value == 0 {
                continue;
            }

            if k == 0 {
                break;
            }

            if (k - idx as i32 * value) >= 0 {
                k -= idx as i32 * value;
                clone_freq[idx] = 0;
            } else {
                let remainder = (idx as i32 * value) - k;
                k = 0;
                clone_freq[idx] = remainder / idx as i32;

                if remainder % idx as i32 > 0 {
                    clone_freq[(remainder % idx as i32) as usize] += 1;
                }
            }
        }

        clone_freq.into_iter().sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![5, 5, 4], 1), 1),
            ((vec![4, 3, 1, 1, 3, 3, 2], 3), 2),
            ((vec![2, 4, 1, 8, 3, 5, 1, 3], 3), 3),
            ((vec![1, 1, 2, 2, 3, 3], 1), 3),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((arr, k), expected))| {
                let result = Solution::find_least_num_of_unique_ints(arr, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

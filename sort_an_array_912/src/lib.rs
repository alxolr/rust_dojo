use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let frequencies = nums.iter().fold(HashMap::new(), |mut acc, it| {
            *acc.entry(it).or_insert(0) += 1;
            acc
        });

        for i in -50000..=50000 {
            if let Some(&frequency) = frequencies.get(&i) {
                for _ in 0..frequency {
                    result.push(i);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0]),
            vec![0, 0, 1, 1, 2, 5]
        );
    }
}

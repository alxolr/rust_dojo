pub struct Solution;

impl Solution {
    fn is_distinct(dictionary: &[i32]) -> bool {
        dictionary.iter().all(|x| *x == 1 || *x == 0)
    }

    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut dictionary = nums.iter().fold(vec![0; 101], |mut acc, item| {
            acc[*item as usize] += 1;

            acc
        });

        let mut operations = 0;
        for chunk in nums.chunks(3) {
            if Self::is_distinct(&dictionary) {
                break;
            }

            operations += 1;

            for it in chunk {
                dictionary[*it as usize] -= 1;
            }
        }

        operations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![1, 2, 3, 4, 2, 3, 3, 5, 7], 2)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::minimum_operations(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

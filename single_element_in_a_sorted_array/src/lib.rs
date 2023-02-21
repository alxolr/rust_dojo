pub struct Solution;

fn find(nums: &[i32], low: usize, high: usize) -> i32 {
    let mid = (low + high) / 2;

    if mid > 0 && nums[mid] == nums[mid - 1] {
        let count = nums[low..=mid].len();

        if count % 2 == 0 {
            return find(nums, mid + 1, high);
        } else {
            return find(nums, low, mid);
        }
    } else if mid < high && nums[mid] == nums[mid + 1] {
        let count = nums[low..=mid + 1].len();

        if count == 3 {
            return nums[low];
        }

        if count % 2 != 0 {
            return find(nums, low, mid + 1);
        } else {
            return find(nums, mid + 2, high);
        }
    } else {
        return nums[mid];
    }
}

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        return find(&nums, 0, nums.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![1, 1, 2, 3, 3, 4, 4, 8, 8], 2),
            (vec![3, 3, 7, 7, 10, 11, 11], 10),
            (
                vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 8, 8, 9, 9, 10, 10, 11, 11],
                5,
            ),
            (vec![0, 1, 1, 2, 2, 3, 3], 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::single_non_duplicate(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

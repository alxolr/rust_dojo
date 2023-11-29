struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let pivot = find_pivot(&nums);

        nums.binary_search(x)

        let first_range = &nums[0..=pivot];
        let second_range = &nums[pivot + 1..nums.len()];

        let idx = if target >= first_range[0] && target <= first_range[first_range.len() - 1] {
            binary_search(first_range, target)
        } else {
            let output: i32 = binary_search(second_range, target);
            if output >= 0 {
                output + pivot as i32 + 1
            } else {
                output
            }
        };

        idx
    }
}

fn binary_search(nums: &[i32], target: i32) -> i32 {
    if nums.len() == 1 {
        if nums[0] == target {
            return 0;
        } else {
            return -1;
        }
    }

    if nums.len() == 0 {
        return -1;
    }

    let mut left = 0usize;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            if mid <= nums.len() - 2 {
                left = mid + 1;
            } else {
                break;
            }
        } else {
            if mid > 0 {
                right = mid - 1;
            } else {
                break;
            }
        }
    }

    -1
}

fn find_pivot(nums: &Vec<i32>) -> usize {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut pivot = left;

    while left < right {
        let mid = (left + right) / 2;

        if nums[left] < nums[mid] {
            left = mid;
        } else if nums[right] > nums[mid] {
            right = mid;
        } else {
            pivot = mid;
            break;
        }
    }

    pivot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_ok() {
        let scenarios = vec![
            // ((vec![4, 5, 6, 7, 0, 1, 2], 0), 4),
            // ((vec![4, 5, 6, 7, 8, 0, 1, 2], 3), -1),
            // ((vec![1], 0), -1),
            // ((vec![1], 1), 0),
            // ((vec![1, 3], 0), -1),
            // ((vec![5, 1, 3], 0), -1),
            ((vec![1, 3, 5], 5), 2),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, target), expected))| {
                let result = Solution::search(nums, target);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

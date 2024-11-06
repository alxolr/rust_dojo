pub struct Solution;

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        // pretty much the idea here is to do bubblesort but with
        // a special condition check to see if the number of bits is equal

        let mut nums = nums;

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                // this is where the bit shifting happens

                if nums[j] < nums[i] {
                    // check if the bit sets are equal

                    if nums[j].count_ones() != nums[i].count_ones() {
                        return false;
                    }

                    // swap the numbers
                    let temp = nums[i];
                    nums[i] = nums[j];
                    nums[j] = temp;
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![8, 4, 2, 30, 15], true),
            (vec![1, 2, 3, 4, 5], true),
            (vec![3, 16, 8, 4, 2], false),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::can_sort_array(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

struct Solution;

impl Solution {
    fn count_water(ml: i32, mr: i32, pt: usize, height: &Vec<i32>) -> i32 {
        let water = i32::min(ml, mr) - height[pt];

        if water < 0 {
            0
        } else {
            water
        }
    }

    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut ans = 0;

        let mut max_left = -1;
        let mut max_right = -1;

        while left < right {
            max_right = max_right.max(height[right]);
            max_left = max_left.max(height[left]);

            let water_left = Self::count_water(max_left, max_right, left, &height);
            let water_right = Self::count_water(max_left, max_right, right, &height);

            ans += water_left + water_right;

            if max_left > max_right {
                right -= 1;
            } else {
                left += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6),
            (vec![4, 2, 0, 3, 2, 5], 9),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::trap(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

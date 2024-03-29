struct Solution;

impl Solution {
    fn area(left: usize, right: usize, heights: &Vec<i32>) -> i32 {
        heights[left].min(heights[right]) * (right - left) as i32
    }

    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0;
        let mut right = heights.len() - 1;

        while left < right {
            let current_area = Self::area(left, right, &heights);
            max_area = max_area.max(current_area);

            if heights[left] >= heights[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49), (vec![1, 1], 1)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_area(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

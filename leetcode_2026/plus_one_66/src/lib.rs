impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let len = digits.len();
        let mut last = len - 1;
        let mut digits = digits;

        let mut remainder = (digits[last] + 1) / 10;
        digits[last] = (digits[last] + 1) % 10;

        while remainder > 0 && last.checked_sub(1).is_some() {
            last -= 1;

            remainder = (digits[last] + 1) / 10;
            digits[last] = (digits[last] + 1) % 10;
        }

        if remainder > 0 {
            digits.insert(0, 1);
        }

        digits
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![1, 2, 3], vec![1, 2, 4]),
            (vec![4, 3, 2, 1], vec![4, 3, 2, 2]),
            (vec![9], vec![1, 0]),
            (vec![9, 9, 9, 9], vec![1, 0, 0, 0, 0]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::plus_one(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

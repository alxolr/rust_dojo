pub struct Solution;

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut counter = 0;
        let mut digits = num;
        
        while digits > 0 {
            let digit = digits % 10;
            digits = digits / 10;

            if num % digit == 0 {
                counter += 1;
            }
        }

        counter
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (121, 2),
            (1248, 4),
            (7, 1),
        ];
        
        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::count_digits(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
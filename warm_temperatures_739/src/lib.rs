pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut result = vec![];

        for entry in temperatures.into_iter().enumerate().rev() {
            if stack.is_empty() {
                result.push(0 as i32);

                stack.push(entry);
            } else {
                // check if it's not empty
                let mut found = false;
                while let Some(last) = stack.last() {
                    if last.1 > entry.1 {
                        found = true;
                        result.push((last.0 - entry.0) as i32);
                        break;
                    } else {
                        stack.pop().unwrap();
                    }
                }

                if !found {
                    result.push(0);
                }

                stack.push(entry);
            }
        }

        result.reverse();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![(
            vec![73, 74, 75, 71, 69, 72, 76, 73],
            vec![1, 1, 4, 2, 1, 1, 0, 0],
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::daily_temperatures(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

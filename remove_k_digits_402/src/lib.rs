pub struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, to_remove: i32) -> String {
        // Convert the input string to a vector of digits
        let digits: Vec<u32> = num.chars().flat_map(|x| x.to_digit(10)).collect();

        // Initialize an empty vector to store the digits of the final number
        let mut stack = vec![];

        // Initialize a counter for the number of digits to remove
        let mut to_remove = to_remove;

        // Iterate over the digits
        for digit in digits {
            // If the stack is empty or we've removed all necessary digits, push the current digit to the stack
            if stack.is_empty() || to_remove == 0 {
                stack.push(digit);
            } else {
                // Otherwise, pop digits from the stack until we find a digit that's not greater than the current one
                // or until we've removed the necessary number of digits
                while let Some(top) = stack.pop() {
                    if top <= digit || to_remove == 0 {
                        stack.push(top);
                        break;
                    } else {
                        to_remove -= 1;
                    }
                }
                // Push the current digit to the stack
                stack.push(digit);
            }
        }

        // Remove digits from the stack 
        if to_remove > 0 {
            for _ in 0..to_remove {
                stack.pop();
            }
        }

        // Convert the digits in the stack to a string, skipping leading zeros
        let number = stack
            .iter()
            .map(|x| x.to_string())
            .skip_while(|x| x == "0")
            .collect::<String>();

        if number.is_empty() {
            return "0".to_string();
        }

        number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (("1432219".to_string(), 3), "1219".to_string()),
            (("10200".to_string(), 1), "200".to_string()),
            (("10".to_string(), 2), "0".to_string()),
            (("9".to_string(), 1), "0".to_string()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((num, k), expected))| {
                let result = Solution::remove_kdigits(num, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

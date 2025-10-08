fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i32, b: i32) -> i32 {
    a / gcd(a, b) * b
}

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        for num in nums.into_iter() {
            stack.push(num);

            if stack.len() > 1 {
                let last = stack.pop().unwrap();
                let pre_last = stack.pop().unwrap();

                let gcd_a_b = gcd(pre_last, last);

                if gcd_a_b > 1 {
                    let lcm_a_b = lcm(pre_last, last);
                    stack.push(lcm_a_b)
                } else {
                    stack.push(pre_last);
                    stack.push(last);
                }
            }
        }
        stack
    }
}

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![6, 4, 3, 2, 7, 6, 2], vec![12, 7, 6]),
            (vec![2, 2, 1, 1, 3, 3, 3], vec![2, 1, 1, 3]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::replace_non_coprimes(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

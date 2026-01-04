use std::collections::HashMap;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();
        nums.into_iter()
            .map(|num| sum_of_exactly_four_divisors(&mut memo, num))
            .sum::<i32>()
    }
}

fn sum_of_exactly_four_divisors(memo: &mut HashMap<i32, i32>, num: i32) -> i32 {
    if let Some(result) = memo.get(&num) {
        return *result;
    }

    let mut count = 2;
    let mut sum = 1 + num;

    for i in 2..=num / 2 {
        if num % i == 0 {
            count += 1;
            sum += i;

            if count > 4 {
                memo.insert(num, 0);
                return 0;
            }
        }
    }

    if count == 4 {
        memo.insert(num, sum);
        return sum;
    }

    return 0;
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![21, 4, 7], 32),
            (vec![21, 21], 64),
            (vec![1, 2, 3, 4, 5], 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::sum_four_divisors(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

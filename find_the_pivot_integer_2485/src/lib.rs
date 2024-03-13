struct Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let total_sum = (n * (1 + n)) / 2;
        let mut prefix_sum = vec![0; n as usize + 1];
        let mut pivot = -1;

        for num in 1..=n {
            let last = prefix_sum[(num - 1) as usize];
            let current = num + last;

            if (current - num) == (total_sum - num) / 2 && (total_sum - num) % 2 == 0 {
                pivot = num;
                break;
            }

            prefix_sum[num as usize] = current;
        }

        pivot
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(4, -1), (8, 6)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::pivot_integer(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

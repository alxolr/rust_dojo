pub struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        (left..=right)
            .map(|n| n.count_ones())
            .filter(Self::is_prime)
            .count() as i32
    }

    fn is_prime(n: &u32) -> bool {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19];

        primes.contains(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{}", 2u32.pow(20))
    }

    #[test]
    fn it_works() {
        let scenarios = vec![((6, 10), 4)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((left, right), expected))| {
                let result = Solution::count_prime_set_bits(left, right);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

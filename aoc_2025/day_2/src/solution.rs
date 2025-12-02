use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::error::Result;
pub struct Solution;

impl Solution {
    fn common_part<F>(input: &str, solution: F) -> Result<u64>
    where
        F: Fn(&u64) -> bool + Send + Sync + Copy, // We need the Send + Sync + Copy in order to use rayon parallel iterator
    {
        Ok(parse_pairs(input)
            .into_par_iter()
            .flat_map(|(start, end)| (start..=end).filter(solution).collect::<Vec<_>>())
            .sum::<u64>())
    }

    pub fn part_1(input: &str) -> Result<u64> {
        Self::common_part(input, is_solution_part_1)
    }

    pub fn part_2(input: &str) -> Result<u64> {
        Self::common_part(input, is_solution_part_2)
    }
}

fn is_solution_part_1(num: &u64) -> bool {
    // The idea is that we transform the num into string.
    // After we check if the length is odd or even, we can't have an odd length number repeating
    // It should always be even length
    // We split the string into two halfs and compare, if they are equal then this is a solution
    // Given: 11221122, Len = 8
    // Mid=4
    // 1122 == 1122 valid solution

    let num = num.to_string();
    let num_len = num.len();

    if !num_len.is_multiple_of(2) {
        return false;
    }

    let mid = num_len / 2;

    num[0..mid] == num[mid..]
}

fn is_solution_part_2(num: &u64) -> bool {
    // Similar as in solution one we transform into string.
    // We get the length.
    // Now we should try to find possible repetitions:
    // We start from 1 range repetion till the half of the string.
    // So if we have 11221122 that will be a string len 8
    // We start at len 1. "1" -> it should be repeated len / range times -> 8 times
    // 1x8 -> "11111111" != 11221122 this is not a solution we move on to range 2.
    // "11"-> should be repeated 8 / 2 => 4 times
    // 11x4 -> "11111111" != 11221122 again not a solution.
    // We move to range 3
    // "112"  -> should be repeated 8 / 3 -> 2 times
    // 112x2 -> "112112" != 11221122 again not solution
    // We move to range 4 the last in our iterator 8/4=2
    // 1122x2 -> "11221122" == 11221122 it's a solution we mark it true

    let num_str = num.to_string();
    let len = num_str.len();

    for range in 1..=len / 2 {
        let times = len / range;
        let mask = (num_str[0..range]).repeat(times);

        if mask == num_str {
            return true;
        }
    }

    false
}

fn parse_pairs(input: &str) -> Vec<(u64, u64)> {
    input
        .split(",")
        .map(|pair| {
            let mut pairs = pair.split("-");
            let left_side = (pairs.next().unwrap()).parse::<u64>().unwrap();
            let right_side = (pairs.next().unwrap()).parse::<u64>().unwrap();

            (left_side, right_side)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const COMMON_INPUT:&'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(Solution::part_1(COMMON_INPUT)?, 1227775554);

        Ok(())
    }

    #[test]
    fn test_is_solution_part_2_ok() {
        let scenarios = vec![
            (11, true),
            (111, true),
            (123123, true),
            (123123123, true),
            (123123122, false),
        ];

        for (idx, (num, expected)) in scenarios.iter().enumerate() {
            assert_eq!(is_solution_part_2(num), *expected);
            println!("scenario {} ({}) ok", idx + 1, num)
        }
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(Solution::part_2(COMMON_INPUT)?, 4174379265);

        Ok(())
    }
}

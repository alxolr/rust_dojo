use std::collections::HashMap;

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<i32> {
        let (mut lefts, mut rights) = lefts_and_rights(input);

        lefts.sort();
        rights.sort();

        let distance = lefts
            .iter()
            .zip(rights.iter())
            .fold(0, |mut acc, (left, right)| {
                acc += (left - right).abs();
                acc
            });

        Ok(distance)
    }

    pub fn part_2(input: &str) -> Result<i32> {
        let (lefts, rights) = lefts_and_rights(input);

        let mut rights_dictionary = HashMap::new();
        for right in rights {
            *rights_dictionary.entry(right).or_insert(0) += 1;
        }

        let similarity = lefts.iter().fold(0, |acc, &left| {
            acc + left * rights_dictionary.get(&left).unwrap_or(&0)
        });

        Ok(similarity)
    }
}

fn lefts_and_rights(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();

    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let mut numbers = line.split_whitespace().flat_map(str::parse::<i32>);

        if let (Some(left), Some(right)) = (numbers.next(), numbers.next()) {
            lefts.push(left);
            rights.push(right);
        } else {
            panic!("wrong input");
        }
    }

    (lefts, rights)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        assert_eq!(super::Solution::part_1(input)?, 11);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        assert_eq!(super::Solution::part_2(input)?, 31);
        Ok(())
    }
}

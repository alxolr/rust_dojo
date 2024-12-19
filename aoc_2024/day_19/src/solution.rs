use std::collections::HashMap;

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<i32> {
        let (patterns, designs) = parse_input(input)?;

        let mut memo: HashMap<(&str, &str), i64> = HashMap::new();
        let valid_designs = designs
            .into_iter()
            .filter(|design| count_combos(&mut memo, design, &patterns) != 0)
            .count();

        Ok(valid_designs as i32)
    }

    pub fn part_2(input: &str) -> Result<i64> {
        let (patterns, designs) = parse_input(input)?;
        let mut memo: HashMap<(&str, &str), i64> = HashMap::new();

        let possible_designs = designs
            .into_iter()
            .map(|design| {
                let combos = count_combos(&mut memo, design, &patterns);

                combos
            })
            .sum();

        Ok(possible_designs)
    }
}

// We try all combinations and once we got the result we add it to the sum
fn count_combos_recursive<'a, 'b, 'c>(
    memo: &mut HashMap<(&'a str, &'b str), i64>,
    curr: &'a str,
    remainder: &'b str,
    patterns: &'c [&'a str],
) -> i64 {
    if let Some(result) = memo.get(&(curr, remainder)) {
        return *result;
    }

    // if we got to the basic scenario then we return 1
    if curr == remainder {
        return 1;
    }

    if remainder.starts_with(curr) {
        // we try all patterns and we sum the all posibilities
        let combos = patterns
            .iter()
            .map(|pattern| {
                count_combos_recursive(memo, *pattern, &remainder[curr.len()..], patterns)
            })
            .sum::<i64>();

        // Memo to not do the overwork again and again
        memo.insert((curr, remainder), combos);

        combos
    } else {
        0
    }
}

fn count_combos<'a, 'b, 'c>(
    memo: &mut HashMap<(&'a str, &'b str), i64>,
    design: &'b str,
    patterns: &'c [&'a str],
) -> i64 {
    count_combos_recursive(memo, "", design, patterns)
}

fn parse_input(input: &str) -> Result<(Vec<&str>, Vec<&str>)> {
    let mut input = input.split("\n\n");
    let patterns = input.next().unwrap().split(", ").collect();

    let designs = input.next().unwrap().lines().collect();

    Ok((patterns, designs))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;
        assert_eq!(Solution::part_1(input)?, 6);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;
        assert_eq!(Solution::part_2(input)?, 16);

        Ok(())
    }
}

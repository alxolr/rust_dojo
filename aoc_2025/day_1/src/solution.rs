use crate::error::Result;

pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<i32> {
        let answer = parse_input(input)
            .scan(50, |state, (dir, count)| {
                let count = count % 100; // We don't need to take in consideration for the full rotations.

                match dir {
                    // If we move Right we increment the sum,and handle the overflow by moding with 100
                    "R" => *state = (*state + count) % 100,
                    // If we move Left we increment the sum with the difference from 100 to count, and then mod with 100
                    _ => *state = (*state + 100 - count) % 100,
                };

                Some(if *state == 0 { 1 } else { 0 })
            })
            .sum();

        Ok(answer)
    }

    pub fn part_2(input: &str) -> Result<i32> {
        let answer = parse_input(input)
            .scan(50, |state, (dir, count)| {
                let mut rotations = 0;
                rotations += count / 100; // Count full rotations
                let count = count % 100;

                match dir {
                    "R" => {
                        let overflow = *state + count;
                        rotations += if overflow != 100 { overflow / 100 } else { 0 }; // For the case when it's 100 we don't double count it
                        *state = overflow % 100;
                    }
                    _ => {
                        let overflow = *state - count; // if we go negative it's a potential overflow considering we did not start at zero
                        rotations += if *state != 0 && overflow < 0 { 1 } else { 0 };
                        *state = (*state + 100 - count) % 100
                    } // If we move Left we increment the sum with the difference from 100 to count, and then mod with 100
                };

                let result = if *state == 0 {
                    rotations + 1
                } else {
                    rotations
                };

                Some(result)
            })
            .sum();

        Ok(answer)
    }
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = (&'a str, i32)> + 'a {
    input.lines().filter(|line| !line.is_empty()).map(|line| {
        let line = line.trim();
        let direction = &line[0..1];
        let times = (&line[1..]).parse::<i32>().expect("Could not parse");

        (direction, times)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82"#;
        assert_eq!(Solution::part_1(input)?, 3);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82"#;
        assert_eq!(Solution::part_2(input)?, 6);

        Ok(())
    }
}

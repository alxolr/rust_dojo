use crate::error::Result;
pub struct Solution;

impl Solution {
    // O(n*m)
    pub fn part_1(input: &str) -> Result<i32> {
        let extract_lines: Vec<Vec<i32>> = extract_lines(input);
        let safe_count = extract_lines
            .iter()
            .filter(|line: &&Vec<i32>| is_safe(line))
            .count() as i32;

        Ok(safe_count)
    }

    // O(n*m^2) -> O(n*m)
    pub fn part_2(input: &str) -> Result<i32> {
        let lines: Vec<Vec<i32>> = extract_lines(input);
        let mut safe_count = 0;

        for line in lines {
            if is_safe(&line) {
                safe_count += 1;
            } else {
                for idx in 0..line.len() {
                    let mut line_clone = line.clone();
                    line_clone.remove(idx);

                    if is_safe(&line_clone) {
                        safe_count += 1;
                        break;
                    }
                }
            }
        }

        Ok(safe_count)
    }
}

fn is_strictly_inc(nums: &[i32]) -> usize {
    nums.windows(2)
        .filter(|w| w[1] - w[0] <= 3 && w[1] > w[0])
        .count()
}

fn is_strictly_dec(nums: &[i32]) -> usize {
    nums.windows(2)
        .filter(|w| w[0] - w[1] <= 3 && w[1] < w[0])
        .count()
}

fn is_safe(line: &[i32]) -> bool {
    let pairs = line.len() - 1;

    let dec_count = is_strictly_dec(line);
    let inc_count = is_strictly_inc(line);

    pairs == dec_count || pairs == inc_count
}

fn extract_lines(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim()
                .split_whitespace()
                .flat_map(|x| x.parse::<i32>())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(super::Solution::part_1(input)?, 2);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(super::Solution::part_2(input)?, 4);

        Ok(())
    }
}
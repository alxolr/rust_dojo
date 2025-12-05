use crate::error::Result;
pub struct Solution;

#[derive(Debug, PartialEq, Eq, Ord)]
struct Interval {
    start: u64,
    end: u64,
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl Interval {
    fn new(start: u64, end: u64) -> Self {
        Interval { start, end }
    }

    fn contains(&self, num: u64) -> bool {
        num >= self.start && num <= self.end
    }

    fn is_overlap(&self, other: &Interval) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    fn overlap(&mut self, other: &Interval) {
        self.start = u64::min(self.start, other.start);
        self.end = u64::max(self.end, other.end);
    }
}

impl Solution {
    pub fn part_1(input: &str) -> Result<i32> {
        let (intervals, ingredients) = parse_input(input);
        let mut sum = 0;

        // For each ingredient we check in the list of intervals
        // Obviously we could've merged the overlapping intervals

        for ingredient in ingredients {
            for interval in intervals.iter() {
                if interval.contains(ingredient) {
                    sum += 1;
                    break; // If I found it in one interval than don't check for others asa it will duplicate
                }
            }
        }

        Ok(sum)
    }

    pub fn part_2(input: &str) -> Result<u64> {
        // The idea is quite simple I sort the Intervals by start, and then use a stack to merge them
        let (mut intervals, _) = parse_input(input);
        intervals.sort_unstable();

        let mut merged_intervals = Vec::<Interval>::with_capacity(intervals.len());

        // I traverse the intervals
        // Keeping the last one on top of the stack, as they are sorted this is guaranteed that if there is an overlap to be
        // on top of the stack.
        for current in intervals {
            if let Some(mut prev) = merged_intervals.pop() {
                if prev.is_overlap(&current) {
                    prev.overlap(&current);
                    merged_intervals.push(prev);
                } else {
                    // if there is no overlap then I push them back both in the same order
                    merged_intervals.push(prev);
                    merged_intervals.push(current);
                }
            } else {
                // For the case when the stack is empty I push it
                merged_intervals.push(current);
            }
        }

        // Simple function to sum the ingredients
        let answer = merged_intervals
            .iter()
            .map(|interval| (interval.end - interval.start) + 1)
            .sum::<u64>();

        Ok(answer)
    }
}

fn parse_input(input: &str) -> (Vec<Interval>, Vec<u64>) {
    let mut parts = input.split("\n\n");
    let intervals = parts.next().unwrap_or_default();
    let ingridients = parts.next().unwrap_or_default();

    let intervals = intervals
        .lines()
        .map(|line| {
            let mut parts = line.trim().split("-");
            let start = parts
                .next()
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap_or_default();

            let end = parts
                .next()
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap_or_default();

            Interval::new(start, end)
        })
        .collect::<Vec<Interval>>();

    let ingredients = ingridients
        .lines()
        .flat_map(|line| line.trim().parse::<u64>())
        .collect();

    (intervals, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32"#;
        assert_eq!(Solution::part_1(input)?, 3);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32"#;
        assert_eq!(Solution::part_2(input)?, 14);

        Ok(())
    }
}

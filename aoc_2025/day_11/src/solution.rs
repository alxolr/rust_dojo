use std::collections::HashMap;

use crate::error::Result;
pub struct Solution;

impl Solution {
    fn dfs<'a>(
        memo: &mut HashMap<&'a str, usize>,
        current: &'a str,
        target: &str,
        graph: &HashMap<&'a str, Vec<&'a str>>,
    ) -> usize {
        if let Some(result) = memo.get(current) {
            return *result;
        }

        if current == target {
            return 1;
        }

        let mut path_count = 0;
        if let Some(children) = graph.get(current) {
            for child in children {
                path_count += Self::dfs(memo, child, target, graph);
            }
        }

        memo.insert(current, path_count);

        path_count
    }

    pub fn part_1(input: &str) -> Result<usize> {
        let graph = parse_input(input);

        let start = "you";
        let end = "out";

        let path_count = Self::dfs(&mut HashMap::new(), start, end, &graph);

        Ok(path_count)
    }

    pub fn part_2(input: &str) -> Result<usize> {
        let graph = parse_input(input);
        // The idea that I do the intervals for the two possible paths
        // I multiply the paths for each interval
        // As we do have two possible paths then the answer will be the sum of the two

        let svr_dac = Self::dfs(&mut HashMap::new(), "svr", "dac", &graph);
        let svr_fft = Self::dfs(&mut HashMap::new(), "svr", "fft", &graph);
        let dac_fft = Self::dfs(&mut HashMap::new(), "dac", "fft", &graph);
        let fft_dac = Self::dfs(&mut HashMap::new(), "fft", "dac", &graph);
        let dac_out = Self::dfs(&mut HashMap::new(), "dac", "out", &graph);
        let fft_out = Self::dfs(&mut HashMap::new(), "fft", "out", &graph);

        // there is svr -> dac -> fft -> out
        let possibilities_path_1 = svr_dac * dac_fft * fft_out;
        // there is svr -> fft -> dac -> out
        let possibilities_path_2 = svr_fft * fft_dac * dac_out;

        Ok(possibilities_path_1 + possibilities_path_2)
    }
}

fn parse_input(input: &'_ str) -> HashMap<&'_ str, Vec<&'_ str>> {
    input.lines().fold(HashMap::new(), |mut acc, line| {
        let mut parts = line.trim().split(": ");

        let parent = parts.next().unwrap_or_default();
        let children = parts
            .next()
            .unwrap_or_default()
            .split_whitespace()
            .collect();

        acc.insert(parent, children);

        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out"#;
        assert_eq!(Solution::part_1(input)?, 5);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#""#;
        assert_eq!(Solution::part_2(input)?, 0);

        Ok(())
    }
}

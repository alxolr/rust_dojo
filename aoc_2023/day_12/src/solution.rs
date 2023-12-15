use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<i64> {
        let structure = structure(input);

        let result = structure.into_iter().map(|line| combos(line)).sum();

        Ok(result)
    }

    pub fn part_2(input: &str) -> Result<i64> {
        let structure = structure(input);

        let result = structure
            .into_iter()
            .enumerate()
            .map(|(idx, line)| {
                let (line, groups) = unfold(line);
                let line = (line.as_str(), groups.as_str());
                let combos = combos(line);
                println!("  ✓ line {} combos {}", idx + 1, combos);

                combos
            })
            .sum();

        Ok(result)
    }
}

fn structure(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let [tiles, numbers]: [&str; 2] =
                l.split_whitespace().collect::<Vec<_>>().try_into().unwrap();

            (tiles, numbers)
        })
        .collect()
}

fn combos(line: (&str, &str)) -> i64 {
    let (data, groups) = line;
    let mut data = data.to_string();

    let valid_paths = valid_paths(&mut data, groups);

    valid_paths
}

fn valid_paths(data: &mut str, groups: &str) -> i64 {
    let size = data.len();

    let groups = groups
        .split(",")
        .map(|it| it.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut solutions = vec!["".to_string()];

    for ch in data.chars() {
        let mut step_solutions = vec![];

        for solution in solutions {
            match ch {
                '?' => {
                    let next_str = format!("{}{}", solution, '#');

                    if has_potential(&next_str, &groups, size) {
                        step_solutions.push(next_str);
                    }

                    let next_str = format!("{}{}", solution, '.');

                    if has_potential(&next_str, &groups, size) {
                        step_solutions.push(next_str);
                    }
                }

                c => {
                    let next_str = format!("{}{}", solution, c);
                    if has_potential(&next_str, &groups, size) {
                        step_solutions.push(next_str);
                    }
                }
            }
        }

        solutions = step_solutions;
    }

    solutions.len() as i64
}

fn has_potential(next_str: &str, groups: &Vec<i32>, size: usize) -> bool {
    let is_final = next_str.len() == size;
    let (clear_groups_count, unclear_count) = count_groups(&next_str, size);

    if is_final {
        return clear_groups_count == *groups;
    }

    let clear_count_equal = groups
        .iter()
        .zip(clear_groups_count.iter())
        .all(|(g, c)| g == c);

    let mut first_unclear_is_less = true;

    if let Some(first_unclear) = groups.iter().skip(clear_groups_count.len()).next() {
        first_unclear_is_less = unclear_count <= *first_unclear;
    }

    let mut min_chars_need = groups
        .iter()
        .skip(clear_groups_count.len())
        .map(|it| it + 1)
        .sum::<i32>();

    if min_chars_need > 0 {
        min_chars_need -= 1;
    }

    let remaining_chars = size - next_str.len();
    let has_enough_chars = remaining_chars >= (min_chars_need - unclear_count) as usize;

    clear_count_equal && first_unclear_is_less && has_enough_chars
}

fn count_groups(s: &str, size: usize) -> (Vec<i32>, i32) {
    let is_final = s.len() == size;
    let mut found_groups: Vec<i32> = vec![];
    let mut unclear_groups_count = 0;
    let mut stack = vec![];

    for ch in s.chars() {
        match ch {
            '#' => stack.push('#'),
            _ => {
                if !stack.is_empty() {
                    found_groups.push(stack.len() as i32);
                }

                stack = vec![];
            }
        }
    }

    if is_final {
        if !stack.is_empty() {
            found_groups.push(stack.len() as i32);
        }
    } else {
        unclear_groups_count = stack.len() as i32;
    }

    (found_groups, unclear_groups_count)
}

fn unfold<'a, 'b>(line: (&'a str, &'b str)) -> (String, String) {
    let (data, groups) = line;

    let data_vec = vec![data; 5];
    let group_vec = vec![groups; 5];

    let data = data_vec.join("?");
    let groups = group_vec.join(",");

    (data, groups)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
        "
        .to_string()
    }

    fn example_two() -> String {
        r"
        .??..??...?##. 1,1,3
        "
        .to_string()
    }

    #[test]
    fn test_structure_ok() -> Result<()> {
        let input = r"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ";

        let expected_structure = vec![("???.###", "1,1,3"), (".??..??...?##.", "1,1,3")];

        assert_eq!(structure(input), expected_structure);

        Ok(())
    }

    #[test]
    fn test_combos_ok() -> Result<()> {
        let scenarios = vec![
            // (("???.###", "1,1,3"), 1),
            // ((".??..??...?##", "1,1,3"), 4),
            // (("?#?#?#?#?#?#?#?", "1,3,1,6"), 1),
            (("????.#...#...", "4,1,1"), 1),
            (("????.######..#####.", "1,6,5"), 4),
            (("?###????????", "3,2,1"), 10),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = combos(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });

        Ok(())
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 21;
        let input = example();
        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }

    #[test]
    fn test_solution_part_2_from_example() -> Result<()> {
        let expected = 525152;
        let input = example();
        assert_eq!(Solution::part_2(&input)?, expected);

        Ok(())
    }
}

use std::{collections::VecDeque, ops::DerefMut};

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{binary_tree::BinaryTree, error::Result};
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<i64> {
        let structure = structure(input);

        let result = structure
            .into_iter()
            .enumerate()
            .par_bridge()
            .map(|(idx, line)| combos(line))
            .sum();

        Ok(result)
    }

    pub fn part_2(input: &str) -> Result<i64> {
        let structure = structure(input);

        let result = structure
            .into_iter()
            .enumerate()
            .par_bridge()
            .map(|(_, line)| {
                let (line, groups) = unfold(line);
                let line = (line.as_str(), groups.as_str());
                combos(line)
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

fn insert(node: Option<&mut Box<BinaryTree>>, val: char) {
    if let Some(node) = node {
        if node.left.is_none() && node.right.is_none() {
            match val {
                '?' => {
                    node.deref_mut().left = Some(BinaryTree::new('#'));
                    node.deref_mut().right = Some(BinaryTree::new('.'));
                }
                c => node.deref_mut().left = Some(BinaryTree::new(c)),
            }
        } else {
            insert(node.left.as_mut(), val);
            insert(node.right.as_mut(), val);
        }
    }
}

fn combos(line: (&str, &str)) -> i64 {
    let (data, groups) = line;

    let mut root = BinaryTree::new(' ');

    for ch in data.chars() {
        insert(Some(&mut root), ch);
    }

    let valid_paths = valid_paths(root, groups, data.len());

    valid_paths
}

fn valid_paths(root: Box<BinaryTree>, groups: &str, size: usize) -> i64 {
    let mut paths = 0;

    let mut nav_queue: VecDeque<(Box<BinaryTree>, String)> = VecDeque::new();

    nav_queue.push_front((root, "".to_string()));

    while !nav_queue.is_empty() {
        let (node, result) = nav_queue.pop_back().unwrap();

        let counted_groups = count_groups(&result, size == result.len());

        if counted_groups == groups {
            paths += 1;
        } else if groups.starts_with(&counted_groups) {
            // continue left and right

            if let Some(left) = node.left {
                let mut str = result.clone();
                str.push(left.value);

                let tuple = (left, str);
                nav_queue.push_front(tuple);
            }

            if let Some(right) = node.right {
                let mut str = result.clone();
                str.push(right.value);

                let tuple = (right, str);
                nav_queue.push_front(tuple);
            }
        }
    }

    paths
}

fn count_groups(s: &str, is_final: bool) -> String {
    let mut valid_groups = vec![];

    let mut stack = vec![];
    for ch in s.chars() {
        match ch {
            '#' => {
                stack.push('#');
            }
            _ => {
                if !stack.is_empty() {
                    valid_groups.push(stack.iter().collect::<String>());
                    stack = vec![];
                }
            }
        }
    }

    if is_final {
        if !stack.is_empty() {
            valid_groups.push(stack.iter().collect::<String>());
        }
    }

    valid_groups
        .iter()
        .map(|x| x.len().to_string())
        .collect::<Vec<_>>()
        .join(",")
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
            (("???.###", "1,1,3"), 1),
            ((".??..??...?##", "1,1,3"), 4),
            (("?#?#?#?#?#?#?#?", "1,3,1,6"), 1),
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

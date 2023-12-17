use regex::Regex;

use crate::error::Result;
pub struct Solution;

type Len = (String, i32);

impl Solution {
    pub fn part_1(input: &str) -> Result<i64> {
        let result: i64 = input.split(",").map(hash).sum::<i64>();

        Ok(result)
    }

    // vec![]

    pub fn part_2(input: &str) -> Result<i32> {
        let regex = Regex::new(r"([a-z]+)([-=])(\d*)").unwrap();

        let hashtable: Vec<Vec<Len>> = input
            .split(",")
            .map(|s| {
                let (_, [label, operation, focal_length]) = regex.captures(s).unwrap().extract();
                let focal_length = focal_length.parse::<i32>().unwrap_or(0);

                (label, operation, focal_length)
            })
            .fold(vec![vec![]; 256], |mut acc, item| {
                let (label, operation, focal_length) = item;

                let index = hash(label) as usize;
                let labels = acc.get_mut(index).unwrap();

                match operation {
                    "=" => {
                        // push the new label to the end if there is none
                        // replace the exinsting label with the new focal len
                        if let Some((idx, _)) =
                            labels.iter().enumerate().find(|(_, (l, _))| l == label)
                        {
                            labels[idx] = (label.to_string(), focal_length);
                        } else {
                            labels.push((label.to_string(), focal_length));
                        }
                    }
                    _ => {
                        // remove the label from the hashtable
                        // fill the empty space, retaining the initial order
                        labels.retain(|(x, _)| label != x);
                    }
                }

                acc
            });

        let result = hashtable
            .iter()
            .enumerate()
            .map(|(box_idx, x)| {
                x.iter().enumerate().fold(0, |mut acc, (idx, (_, val))| {
                    acc += (box_idx as i32 + 1) * (idx as i32 + 1) * (*val);

                    acc
                })
            })
            .sum::<i32>();

        Ok(result)
    }
}

fn hash(s: &str) -> i64 {
    let mut val: usize = 0;

    for ch in s.bytes() {
        val += ch as usize;
        val *= 17;
        val %= 256;
    }

    val as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 1320;
        let input = example();

        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }

    #[test]
    fn test_solution_part_2_from_example() -> Result<()> {
        let expected = 145;
        let input = example();

        assert_eq!(Solution::part_2(&input)?, expected);

        Ok(())
    }
}

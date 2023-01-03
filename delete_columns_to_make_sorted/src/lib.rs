pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let m = strs[0].len();

        let mut rotated = vec![vec!['.'; n]; m];

        for i in 0..n {
            for j in 0..m {
                let chs = strs[i].chars();
                let value = chs.peekable().nth(j).unwrap();

                rotated[j][i] = value;
            }
        }

        rotated
            .into_iter()
            .enumerate()
            .map(|(idx, item)| {
                let str = item.iter().collect::<String>();

                (idx, str)
            })
            .filter(|(_, item)| {
                let mut cloned = item.chars().collect::<Vec<_>>();
                cloned.sort();

                let clonned_str = cloned.iter().collect::<String>();

                if clonned_str.as_str() == item {
                    false
                } else {
                    true
                }
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                vec!["cba".to_string(), "daf".to_string(), "ghi".to_string()],
                1,
            ),
            (vec!["a".to_string(), "b".to_string()], 0),
            (
                vec!["zyx".to_string(), "wvu".to_string(), "tsr".to_string()],
                3,
            ),
            (
                vec!["rrjk".to_string(), "furt".to_string(), "guzm".to_string()],
                2,
            ),
            (vec!["x".to_string(), "q".to_string()], 1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::min_deletion_size(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

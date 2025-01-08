pub struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let net_shifts = shifts.into_iter().fold(vec![0; s.len()], |mut acc, shift| {
            let [start, end, direction] = shift[..] else {
                panic!("wrong format")
            };

            (start..=end).for_each(|idx| match direction {
                0 => acc[idx as usize] -= 1,
                _ => acc[idx as usize] += 1,
            });

            acc
        });

        let result = s
            .bytes()
            .enumerate()
            .map(|(idx, byte)| {
                let shifts = (net_shifts[idx] % 26 + 26) % 26;
                let letter_idx = ((byte - b'a') + shifts as u8) % 26;
                
                (letter_idx + b'a') as char
            })
            .collect();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (
                    "abc".to_string(),
                    vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]],
                ),
                "ace".to_string(),
            ),
            (
                ("dztz".to_string(), vec![vec![0, 0, 0], vec![1, 1, 1]]),
                "catz".to_string(),
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((s, shifts), expected))| {
                let result = Solution::shifting_letters(s, shifts);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

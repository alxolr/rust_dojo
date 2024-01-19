pub struct Solution;

impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let alphabet = "ABCDEFGJKLMNOPQRSTUVWXYZ".chars();
        let mut result = vec![];

        let cells = s.split(":").collect::<Vec<&str>>();

        let cell_one = cells[0]; // A1
        let cell_two = cells[1]; // B5

        let start_col_range = &cell_one[0..=0]; // A
        let start_row_range = cell_one[1..=1].parse::<usize>().unwrap(); // 1

        let finish_col_range = &cell_two[0..=0]; // B
        let finish_row_range = cell_two[1..=1].parse::<usize>().unwrap(); // 5


        let mut range_started = false;
        for letter in alphabet {
            if letter.to_string() == start_col_range {
                range_started = true;
            }

            if range_started {
                (start_row_range..=finish_row_range)
                    .for_each(|row| result.push(format!("{}{}", letter, row)));
            }

            if letter.to_string() == finish_col_range {
                range_started = false;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (
                "A1:F1".to_string(),
                vec![
                    "A1".to_string(),
                    "B1".to_string(),
                    "C1".to_string(),
                    "D1".to_string(),
                    "E1".to_string(),
                    "F1".to_string(),
                ],
            ),
            (
                "K1:L2".to_string(),
                vec![
                    "K1".to_string(),
                    "K2".to_string(),
                    "L1".to_string(),
                    "L2".to_string(),
                ],
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::cells_in_range(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

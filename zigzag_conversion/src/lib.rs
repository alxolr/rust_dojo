use core::num;

struct Solution;

enum Direction {
    Down,
    Diagonal,
}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let count_cols = |len: usize, rows: i32| {
            let cols_square = rows - 1;
            let fit = rows + rows - 2;

            let num_squares = (len as f32 / fit as f32).ceil() as i32;

            (cols_square * num_squares).max(1)
        };

        let num_cols = count_cols(s.len(), num_rows);

        let mut res = vec![vec![' '; num_cols as usize]; num_rows as usize];

        let mut i = 0;
        let mut j = 0;
        let mut visited = 0;
        let mut direction = Direction::Down;

        for c in s.chars() {
            res[i][j] = c;
            visited += 1;

            direction = if change_direction(visited, num_rows, &direction) {
                match direction {
                    Direction::Down => Direction::Diagonal,
                    Direction::Diagonal => {
                        visited = 1;
                        Direction::Down
                    }
                }
            } else {
                direction
            };

            match direction {
                Direction::Down => {
                    i += 1;
                }
                Direction::Diagonal => {
                    i -= 1;
                    j += 1;
                }
            }
        }

        let ans = res
            .iter()
            .map(|row| row.iter().filter(|it| it != &&' ').collect::<String>())
            .collect::<String>();

        ans
    }
}

fn change_direction(visited: i32, num_rows: i32, direction: &Direction) -> bool {
    match direction {
        Direction::Down => {
            if visited < num_rows {
                false
            } else {
                true
            }
        }
        Direction::Diagonal => {
            if visited < (num_rows + num_rows - 2) + 1 {
                false
            } else {
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (("A".to_string(), 1), "A".to_string()),
            (
                ("PAYPALISHIRING".to_string(), 3),
                "PAHNAPLSIIGYIR".to_string(),
            ),
            (
                ("PAYPALISHIRING".to_string(), 4),
                "PINALSIGYAHRPI".to_string(),
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((s, r), expected))| {
                let result = Solution::convert(s, r);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

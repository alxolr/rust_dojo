use std::collections::HashSet;

struct Solution;

fn fibo(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    fibo(n - 1) + fibo(n - 2)
}

fn validate(range: &[char]) -> bool {
    let digits = range
        .iter()
        .filter(|c| c.is_digit(10))
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let distinct_digits = digits.iter().collect::<HashSet<_>>();

    let size_check = distinct_digits.len() == digits.len();
    let in_between = digits.iter().all(|x| x >= &1 && x <= &9);

    size_check && in_between
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let rows = board.len();
        let cols = board[0].len();

        if rows != 9 && cols != 9 {
            return false;
        }

        let is_valid_rows = board.iter().all(|row| validate(row));

        let mut is_valid_columns = true;

        for row in 0..rows {
            let mut columns: Vec<char> = vec![];
            for col in 0..cols {
                columns.push(board[col][row]);
            }

            is_valid_columns = is_valid_columns && validate(&columns);
        }

        let boxes = vec![(0usize..3), (3..6), (6..9)];

        let mut is_valid_boxes = true;
        for i in 0..boxes.len() {
            for j in 0..boxes.len() {
                let rows = boxes[i].clone();
                let cols = boxes[j].clone();

                let mut records: Vec<char> = vec![];
                for row in rows {
                    for col in cols.clone() {
                        records.push(board[row][col]);
                    }
                }
                is_valid_boxes = is_valid_boxes && validate(&records);
            }
        }

        is_valid_rows && is_valid_columns && is_valid_boxes
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(
            vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            true,
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::is_valid_sudoku(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn test_things_work() {}
}

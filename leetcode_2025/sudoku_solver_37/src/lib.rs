impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        solve_sudoku(board);
    }
}

fn solve_sudoku(board: &mut Vec<Vec<char>>) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == '.' {
                for c in '1'..='9' {
                    if is_valid(board, row, col, c) {
                        board[row][col] = c;
                        if solve_sudoku(board) {
                            return true;
                        }
                        board[row][col] = '.'; // backtrack
                    }
                }
                return false; // no valid digit found
            }
        }
    }
    true // solved
}

fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, c: char) -> bool {
    for i in 0..9 {
        if board[row][i] == c || board[i][col] == c {
            return false;
        }
        let box_row = 3 * (row / 3) + i / 3;
        let box_col = 3 * (col / 3) + i % 3;
        if board[box_row][box_col] == c {
            return false;
        }
    }
    true
}

pub struct Solution;

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
            vec![
                vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
            ],
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (mut input, expected))| {
                Solution::solve_sudoku(&mut input);
                assert_eq!(input, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

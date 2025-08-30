use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); 9];

        board
            .into_iter()
            .enumerate()
            .all(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .filter(|(_, c)| *c != '.' && c.is_ascii_digit())
                    .all(|(j, c)| {
                        let box_idx = (i / 3) * 3 + (j / 3);
                        
                        rows[i].insert(c) && 
                        cols[j].insert(c) && 
                        boxes[box_idx].insert(c)
                    })
            })
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
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
            ),
            (
                vec![
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '3', '.', '.'],
                    vec!['.', '.', '.', '1', '8', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '1', '.', '9', '7', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '3', '6', '.', '1', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '2', '.'],
                ],
                true,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::is_valid_sudoku(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

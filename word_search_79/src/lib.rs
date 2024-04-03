use std::collections::HashSet;

pub struct Solution;

impl Solution {
    fn dfs(
        row: usize,
        col: usize,
        i: usize,
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
        path: &mut HashSet<(usize, usize)>,
    ) -> bool {
        let cols = board[0].len();
        let rows = board.len();

        if i == word.len() {
            return true;
        }

        if col >= cols || row >= rows || word[i] != board[row][col] || path.contains(&(row, col)) {
            return false;
        }

        path.insert((row, col));

        let up = if row.checked_sub(1).is_some() {
            Self::dfs(row - 1, col, i + 1, board, word, path)
        } else {
            false
        };

        let left = if col.checked_sub(1).is_some() {
            Self::dfs(row, col - 1, i + 1, board, word, path)
        } else {
            false
        };
        let down = Self::dfs(row + 1, col, i + 1, board, word, path);
        let right = Self::dfs(row, col + 1, i + 1, board, word, path);

        let result = left || up || right || down;

        path.remove(&(row, col));

        result
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.chars().collect::<Vec<_>>();

        for row in 0..board.len() {
            for col in 0..board[0].len() {
                if Self::dfs(row, col, 0, &board, &word, &mut HashSet::new()) == true {
                    return true;
                }
            }
        }

        false
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
                    vec![
                        vec!['A', 'B', 'C', 'E'],
                        vec!['S', 'F', 'C', 'S'],
                        vec!['A', 'D', 'E', 'E'],
                    ],
                    "ABCCED".to_string(),
                ),
                true,
            ),
            (
                (
                    vec![
                        vec!['A', 'B', 'C', 'E'],
                        vec!['S', 'F', 'C', 'S'],
                        vec!['A', 'D', 'E', 'E'],
                    ],
                    "SEE".to_string(),
                ),
                true,
            ),
            (
                (
                    vec![
                        vec!['A', 'B', 'C', 'E'],
                        vec!['S', 'F', 'C', 'S'],
                        vec!['A', 'D', 'E', 'E'],
                    ],
                    "ABCB".to_string(),
                ),
                false,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((board, word), expected))| {
                let result = Solution::exist(board, word);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

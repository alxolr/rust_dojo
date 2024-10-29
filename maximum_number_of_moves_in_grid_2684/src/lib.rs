use std::collections::HashMap;

pub struct Solution;

/// Problem Maximum Number of Moves in a Grid 2684
/// https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/

/// Description:
/// You can move only on the right, a row up, same, or down.
/// Your next cell should be strictly bigger than current cell
/// Return the maximum number of moves you could make in the grid.

impl Solution {
    fn dp(
        memo: &mut HashMap<(usize, usize), i32>,
        row: usize,
        col: usize,
        grid: &Vec<Vec<i32>>,
    ) -> i32 {
        if let Some(result) = memo.get(&(row, col)) {
            return *result;
        }

        if col + 1 >= grid[0].len() {
            return 0;
        }

        let mut options = vec![];
        let current = grid[row][col];

        // Check moving up
        if row > 0 && grid[row - 1][col + 1] > current {
            options.push(1 + Self::dp(memo, row - 1, col + 1, &grid));
        }

        // Check moving right
        if grid[row][col + 1] > current {
            options.push(1 + Self::dp(memo, row, col + 1, &grid));
        }

        // Check moving down
        if row < grid.len() - 1 && grid[row + 1][col + 1] > current {
            options.push(1 + Self::dp(memo, row + 1, col + 1, &grid));
        }

        let result = options.into_iter().max().unwrap_or(0);

        // Cache the result in memo
        memo.insert((row, col), result);

        return result;
    }

    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut memo = HashMap::new();
        let first_col = 0;
        let mut max = 0;

        for row in 0..grid.len() {
            max = max.max(Self::dp(&mut memo, row, first_col, &grid))
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                vec![
                    vec![2, 4, 3, 5],
                    vec![5, 4, 9, 3],
                    vec![3, 4, 2, 11],
                    vec![10, 9, 13, 15],
                ],
                3,
            ),
            (vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]], 0),
            (
                vec![
                    vec![187, 167, 209, 251, 152, 236, 263, 128, 135],
                    vec![267, 249, 251, 285, 73, 204, 70, 207, 74],
                    vec![189, 159, 235, 66, 84, 89, 153, 111, 189],
                    vec![120, 81, 210, 7, 2, 231, 92, 128, 218],
                    vec![193, 131, 244, 293, 284, 175, 226, 205, 245],
                ],
                3,
            ),
            (
                vec![
                    vec![
                        1000000, 92910, 1021, 1022, 1023, 1024, 1025, 1026, 1027, 1028, 1029, 1030,
                        1031, 1032, 1033, 1034, 1035, 1036, 1037, 1038, 1039, 1040, 1041, 1042,
                        1043, 1044, 1045, 1046, 1047, 1048, 1049, 1050, 1051, 1052, 1053, 1054,
                        1055, 1056, 1057, 1058, 1059, 1060, 1061, 1062, 1063, 1064, 1065, 1066,
                        1067, 1068,
                    ],
                    vec![
                        1069, 1070, 1071, 1072, 1073, 1074, 1075, 1076, 1077, 1078, 1079, 1080,
                        1081, 1082, 1083, 1084, 1085, 1086, 1087, 1088, 1089, 1090, 1091, 1092,
                        1093, 1094, 1095, 1096, 1097, 1098, 1099, 1100, 1101, 1102, 1103, 1104,
                        1105, 1106, 1107, 1108, 1109, 1110, 1111, 1112, 1113, 1114, 1115, 1116,
                        1117, 1118,
                    ],
                ],
                49,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_moves(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

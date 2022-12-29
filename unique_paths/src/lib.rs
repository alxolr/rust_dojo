pub struct Solution;

fn dfs(grid: &mut Vec<Vec<i32>>, x: isize, y: isize, mut solution: i32) -> i32 {
    if x < 0 || y < 0 {
        return 0;
    }

    let x = x as usize;
    let y = y as usize;

    if x >= grid.len() || y >= grid[0].len() || grid[x][y] == -1 {
        0
    } else if grid[x][y] == 2 {
        if solution == -1 {
            1
        } else {
            0
        }
    } else {
        grid[x][y] = -1;
        solution -= 1;

        let total_solutions = dfs(grid, x as isize + 1, y as isize, solution)
            + dfs(grid, x as isize - 1, y as isize, solution)
            + dfs(grid, x as isize, y as isize + 1, solution)
            + dfs(grid, x as isize, y as isize - 1, solution);

        grid[x][y] = 0;

        total_solutions
    }
}

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut graph = grid.clone();
        let mut solution = 0;
        let mut start_x = 0;
        let mut start_y = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if graph[i][j] == 0 {
                    solution += 1;
                } else if grid[i][j] == 1 {
                    start_x = i as isize;
                    start_y = j as isize;
                }
            }
        }

        dfs(&mut graph, start_x, start_y, solution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]];
        let result = Solution::unique_paths_iii(grid);
        assert_eq!(result, 4);
    }
}

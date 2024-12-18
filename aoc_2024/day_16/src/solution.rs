use std::collections::{BinaryHeap, VecDeque};

use crate::error::Result;
pub struct Solution;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Dir {
    East,
    North,
    South,
    West,
}

#[derive(PartialEq, Eq)]
struct Activity {
    cost: i32,
    dir: Dir,
    pos: (usize, usize),
}

impl Ord for Activity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // Min-heap
    }
}

impl PartialOrd for Activity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn min_cost_path(
    grid: &[Vec<char>],
    start: (usize, usize),
    end: (usize, usize),
    dp: &mut Vec<Vec<i32>>,
    prev: &mut Vec<Vec<VecDeque<Option<(isize, isize)>>>>,
) -> Option<i32> {
    let rows = grid.len();
    let cols = grid[0].len();

    if start.0 >= rows || start.1 >= cols || end.0 >= rows || end.1 >= cols {
        return None;
    }

    dp[start.0][start.1] = 0; // start the cost at zero
    prev[start.0][start.1].push_back(None); // there are no previous nodes for start

    let mut pq = BinaryHeap::new();
    pq.push(Activity {
        cost: 0,
        pos: start,
        dir: Dir::East,
    });

    while !pq.is_empty() {
        let Activity {
            cost,
            pos: (x, y),
            dir,
        } = pq.pop().unwrap();

        if (x, y) == end {
            return Some(cost);
        }

        match dir {
            Dir::East => {
                let actions = [
                    (1, Dir::East, (0, 1)),
                    (1001, Dir::North, (-1, 0)),
                    (1001, Dir::South, (1, 0)),
                    (2001, Dir::West, (0, -1)),
                ];

                apply_actions(actions, x, y, grid, cost, dp, &mut pq, prev);
            }
            Dir::North => {
                let actions = [
                    (1, Dir::North, (-1, 0)),
                    (1001, Dir::East, (0, 1)),
                    (1001, Dir::West, (0, -1)),
                    (2001, Dir::South, (1, 0)),
                ];

                apply_actions(actions, x, y, grid, cost, dp, &mut pq, prev);
            }
            Dir::South => {
                let actions = [
                    (1, Dir::South, (1, 0)),
                    (1001, Dir::East, (0, 1)),
                    (1001, Dir::West, (0, -1)),
                    (2001, Dir::North, (-1, 0)),
                ];

                apply_actions(actions, x, y, grid, cost, dp, &mut pq, prev);
            }
            Dir::West => {
                let actions = [
                    (1, Dir::West, (0, -1)),
                    (1001, Dir::North, (-1, 0)),
                    (1001, Dir::South, (1, 0)),
                    (2001, Dir::East, (0, 1)),
                ];

                apply_actions(actions, x, y, grid, cost, dp, &mut pq, prev);
            }
        }
    }

    Some(dp[end.0][end.1])
}

fn apply_actions(
    actions: [(i32, Dir, (isize, isize)); 4],
    row: usize,
    col: usize,
    grid: &[Vec<char>],
    cost: i32,
    dp: &mut Vec<Vec<i32>>,
    pq: &mut BinaryHeap<Activity>,
    prev: &mut Vec<Vec<VecDeque<Option<(isize, isize)>>>>,
) {
    for (current_cost, dir, (dx, dy)) in actions {
        let new_x = row as isize + dx;
        let new_y = col as isize + dy;

        match safe_get(new_x, new_y, grid) {
            Some(object) => {
                if ['.', 'E'].contains(&object) {
                    let new_cost = cost + current_cost;

                    let new_x = new_x as usize;
                    let new_y = new_y as usize;

                    if new_cost <= dp[new_x as usize][new_y as usize] {
                        // we need to check the equal costs
                        // if it's strictly lower
                        if new_cost < dp[new_x as usize][new_y as usize] {
                            prev[new_x][new_y].clear();
                        }
                        
                        let (prev_row, prev_rest) = prev.split_at_mut(row + 1);
                        let prev_row = &mut prev_row[row];
                        let prev_col = &mut prev_row[col];
                        let prev_new = &mut prev_rest[0][new_y];
                        
                        for p in prev_col.iter_mut() {
                            prev_new.push_back(*p);
                        }
                        // add the new parents
                        prev[new_x][new_y].push_back(Some((row as isize, col as isize)));
                        // add new cost
                        dp[new_x as usize][new_y as usize] = new_cost;
                        pq.push(Activity {
                            pos: (new_x, new_y),
                            dir,
                            cost: new_cost,
                        });
                    }
                }
            }
            _ => {}
        }
    }
}

fn safe_get(row: isize, col: isize, grid: &[Vec<char>]) -> Option<char> {
    if row < 0 || col < 0 {
        return None;
    }
    let row = row as usize;
    let col = col as usize;
    if row >= grid.len() || col >= grid[0].len() {
        return None;
    }

    Some(grid[row][col])
}

impl Solution {
    pub fn part_1(input: &str) -> Result<i32> {
        let grid = load_grid(input);
        let start = find_point(&grid, 'S').unwrap();
        let end = find_point(&grid, 'E').unwrap();
        let rows = grid.len();
        let cols = grid[0].len();

        let mut dp = vec![vec![i32::MAX; cols]; rows];
        let mut prev = vec![vec![VecDeque::new(); cols]; rows];
        let min_cost = min_cost_path(&grid, start, end, &mut dp, &mut prev);

        Ok(min_cost.unwrap())
    }

    pub fn part_2(input: &str) -> Result<i32> {
        let grid = load_grid(input);
        let start = find_point(&grid, 'S').unwrap();
        let end = find_point(&grid, 'E').unwrap();
        let rows = grid.len();
        let cols = grid[0].len();

        let mut costs = vec![vec![i32::MAX; cols]; rows];
        let mut prev = vec![vec![VecDeque::new(); cols]; rows];
        min_cost_path(&grid, start, end, &mut costs, &mut prev);

        let mut paths = Vec::new();

        if costs[end.0][end.1] != i32::MAX {
            backtrack(
                &costs,
                &prev,
                (end.0 as isize, end.1 as isize),
                (start.0 as isize, start.1 as isize),
                &mut vec![(end.0 as isize, end.1 as isize)],
                costs[end.0][end.1],
                &mut paths,
            );
        }

        println!("{:?}", paths);

        Ok((1))
    }
}

fn backtrack(
    grid: &[Vec<i32>],
    prev: &[Vec<VecDeque<Option<(isize, isize)>>>],
    current: (isize, isize),
    source: (isize, isize),
    path: &mut Vec<(isize, isize)>,
    min_cost: i32,
    paths: &mut Vec<Vec<(isize, isize)>>,
) {
    if current == source {
        if path
            .iter()
            .map(|&(r, c)| grid[r as usize][c as usize])
            .sum::<i32>()
            == min_cost
        {
            paths.push(path.clone());
        }
        return;
    }

    for &p in &prev[current.0 as usize][current.1 as usize] {
        if let Some(prev_node) = p {
            path.push(prev_node);
            backtrack(grid, prev, prev_node, source, path, min_cost, paths);
            path.pop();
        }
    }
}

fn find_point(grid: &[Vec<char>], point: char) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == point) {
            return Some((i, j));
        }
    }
    None
}

fn load_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
        assert_eq!(Solution::part_1(input)?, 11048);

        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        assert_eq!(Solution::part_1(input)?, 7036);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        assert_eq!(Solution::part_2(input)?, 45);

        Ok(())
    }
}

use std::collections::HashSet;

use crate::error::Result;
pub struct Solution;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

type Point = (usize, usize);
type PointDir = (Point, Direction, usize);

fn compatibility_dir(dir: &Direction) -> &'static str {
    match dir {
        Direction::Left => "-LF",
        Direction::Right => "J-7",
        Direction::Down => "L|J",
        Direction::Up => "F|7",
    }
}

impl Solution {
    pub fn process(grid: &Vec<Vec<char>>, n: usize, m: usize) -> (usize, HashSet<Point>) {
        let start = 'S';

        let mut numbered_grid = vec![vec![n * m + 1; m]; n];
        let mut start_point: (usize, usize) = (0, 0);

        let mut max_distance = 0;
        let mut cycle_path: HashSet<Point> = HashSet::new();

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == start {
                    start_point.0 = i;
                    start_point.1 = j;

                    numbered_grid[i][j] = 0;
                    break;
                }
            }
        }

        cycle_path.insert(start_point);

        let mut stack = vec![
            maybe_point(Direction::Up, start_point, n, m, 1),
            maybe_point(Direction::Down, start_point, n, m, 1),
            maybe_point(Direction::Left, start_point, n, m, 1),
            maybe_point(Direction::Right, start_point, n, m, 1),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<PointDir>>();

        while !stack.is_empty() {
            let (point, dir, distance): PointDir = stack.pop().unwrap();
            let mut distance = distance;
            let item = grid[point.0][point.1];

            if compatibility_dir(&dir).contains(item) {
                cycle_path.insert(point);
                distance = numbered_grid[point.0][point.1].min(distance);
                numbered_grid[point.0][point.1] = distance;
                max_distance = max_distance.max(distance);

                let up = || maybe_point(Direction::Up, point, n, m, distance + 1);
                let down = || maybe_point(Direction::Down, point, n, m, distance + 1);
                let left = || maybe_point(Direction::Left, point, n, m, distance + 1);
                let right = || maybe_point(Direction::Right, point, n, m, distance + 1);

                let maybe_next_point = match dir {
                    Direction::Left => match item {
                        'F' => down(),
                        '-' => left(),
                        'L' => up(),
                        _ => panic!("shouldn't match this arm"),
                    },
                    Direction::Right => match item {
                        'J' => up(),
                        '-' => right(),
                        '7' => down(),
                        _ => panic!("shouldn't match this arm"),
                    },
                    Direction::Down => match item {
                        'L' => right(),
                        '|' => down(),
                        'J' => left(),
                        _ => panic!("shouldn't match this arm"),
                    },
                    Direction::Up => match item {
                        'F' => right(),
                        '|' => up(),
                        '7' => left(),
                        _ => panic!("shouldn't match this arm"),
                    },
                };

                if let Some(next_point) = maybe_next_point {
                    stack.push(next_point);
                }
            }
        }

        (max_distance, cycle_path)
    }

    pub fn part_1(input: &str) -> Result<i32> {
        let grid = gridify(input);
        let n = grid.len();
        let m = grid[0].len();
        let (max_distance, _) = Solution::process(&grid, n, m);

        Ok((max_distance + 1) as i32 / 2)
    }

    pub fn part_2(input: &str) -> Result<i32> {
        let grid = gridify(input);
        let mut debug_grid = grid.clone();


        let n = grid.len();
        let m = grid[0].len();
        let (_, cycle_path) = Solution::process(&grid, n, m);

        let mut count_tiles = 0;

        for i in 0..n {
            for j in 0..m {
                if cycle_path.contains(&(i, j)) {
                    debug_grid[i][j] = match grid[i][j] {
                        'F' => '┌',
                        'J' => '┘',
                        '7' => '┐',
                        'L' => '└',
                        '-' => '─',
                        '|' => '│',
                        c => c,
                    }
                }

                if grid[i][j] == '.' {
                    let is_bound = is_bound(Direction::Up, (i, j), &grid, &cycle_path, n, m)
                        && is_bound(Direction::Down, (i, j), &grid, &cycle_path, n, m)
                        && is_bound(Direction::Left, (i, j), &grid, &cycle_path, n, m)
                        && is_bound(Direction::Right, (i, j), &grid, &cycle_path, n, m);

                    if is_bound {
                        count_tiles += 1;

                        debug_grid[i][j] = 'I';
                    } else {
                        debug_grid[i][j] = 'O';
                    }
                }
            }
        }

        for i in 0..n {
            println!("{}", debug_grid[i].iter().collect::<String>());
        }

        Ok(count_tiles)
    }
}

fn is_bound(
    dir: Direction,
    point: Point,
    grid: &Vec<Vec<char>>,
    cycle_path: &HashSet<Point>,
    n: usize,
    m: usize,
) -> bool {
    let (i, j) = point;

    match dir {
        Direction::Left => {
            let mut is_bound = false;

            let mut mj = j;
            loop {
                if mj > 0 {
                    mj -= 1;

                    if grid[i][mj] != '.' {
                        if cycle_path.contains(&(i, mj)) {
                            is_bound = true;
                        }

                        break;
                    }
                } else {
                    break;
                }
            }

            is_bound
        }
        Direction::Right => {
            let mut is_bound = false;

            let mut mj = j;
            loop {
                if mj < m - 1 {
                    mj += 1;

                    if grid[i][mj] != '.' {
                        if cycle_path.contains(&(i, mj)) {
                            is_bound = true;
                        }

                        break;
                    }
                } else {
                    break;
                }
            }

            is_bound
        }
        Direction::Down => {
            let mut is_bound = false;

            let mut mi = i;
            loop {
                if mi < n - 1 {
                    mi += 1;

                    if grid[mi][j] != '.' {
                        if cycle_path.contains(&(mi, j)) {
                            is_bound = true;
                        }

                        break;
                    }
                } else {
                    break;
                }
            }

            is_bound
        }
        Direction::Up => {
            let mut is_bound = false;

            let mut mi = i;
            loop {
                if mi > 0 {
                    mi -= 1;

                    if grid[mi][j] != '.' {
                        if cycle_path.contains(&(mi, j)) {
                            is_bound = true;
                        }

                        break;
                    }
                } else {
                    break;
                }
            }

            is_bound
        }
    }
}

fn maybe_point(
    next_direction: Direction,
    curr_point: (usize, usize),
    n: usize,
    m: usize,
    distance: usize,
) -> Option<PointDir> {
    match next_direction {
        Direction::Left => {
            if curr_point.1 > 0 {
                let point: PointDir = ((curr_point.0, curr_point.1 - 1), Direction::Left, distance);
                Some(point)
            } else {
                None
            }
        }
        Direction::Right => {
            if curr_point.1 < m - 1 {
                let point: PointDir =
                    ((curr_point.0, curr_point.1 + 1), Direction::Right, distance);

                Some(point)
            } else {
                None
            }
        }
        Direction::Down => {
            if curr_point.0 < n - 1 {
                let point = ((curr_point.0 + 1, curr_point.1), next_direction, distance);
                Some(point)
            } else {
                None
            }
        }
        Direction::Up => {
            if curr_point.0 > 0 {
                Some(((curr_point.0 - 1, curr_point.1), next_direction, distance))
            } else {
                None
            }
        }
    }
}

fn gridify(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
        "
        .to_string()
    }

    fn part_two_example() -> String {
        r"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
        "
        .to_string()
    }

    #[test]
    fn test_input_into_matrix_char() -> Result<()> {
        let input = r"-L|F7
        7S-7|
        L|7||";

        let expected_grid = vec![
            vec!['-', 'L', '|', 'F', '7'],
            vec!['7', 'S', '-', '7', '|'],
            vec!['L', '|', '7', '|', '|'],
        ];

        assert_eq!(gridify(input), expected_grid);

        Ok(())
    }

    #[test]
    fn test_solution_part_2_from_example() -> Result<()> {
        let expected = 4;
        let input = part_two_example();
        assert_eq!(Solution::part_2(&input)?, expected);

        Ok(())
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 8;
        let input = example();
        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }
}

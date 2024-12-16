use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<i32> {
        let (mut grid, movements) = load_input(input);

        let mut robot_position = find_robot_position(&grid);

        for movement in movements {
            let (row, col) = robot_position;
            match movement {
                '^' => {
                    if let Some(obj) = get(row - 1, col, &grid) {
                        match obj {
                            '.' => {
                                move_item((row, col), (row - 1, col), '@', &mut grid);
                                robot_position = (row - 1, col);
                            }
                            'O' => {
                                let mut start_row = row - 1;
                                let mut stacked_items =
                                    vec![(row, col, '@'), (start_row, col, 'O')];

                                while start_row >= 0 {
                                    start_row -= 1;

                                    if let Some(item) = get(start_row, col, &grid) {
                                        match item {
                                            '.' => {
                                                for (idx, &(item_row, item_col, item_char)) in
                                                    stacked_items.iter().rev().enumerate()
                                                {
                                                    move_item(
                                                        (item_row, item_col),
                                                        (start_row + idx as isize, item_col),
                                                        item_char,
                                                        &mut grid,
                                                    );

                                                    if item_char == '@' {
                                                        // update robot position
                                                        robot_position =
                                                            (start_row + idx as isize, item_col)
                                                    }
                                                }
                                                break;
                                            }
                                            'O' => {
                                                stacked_items.push((start_row, col, 'O'));
                                            }
                                            _ => break,
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                'v' => {
                    if let Some(obj) = get(row + 1, col, &grid) {
                        match obj {
                            '.' => {
                                move_item((row, col), (row + 1, col), '@', &mut grid);
                                robot_position = (row + 1, col);
                            }
                            'O' => {
                                let mut start_row = row + 1;
                                let mut stacked_items =
                                    vec![(row, col, '@'), (start_row, col, 'O')];

                                while start_row >= 0 {
                                    start_row += 1;

                                    if let Some(item) = get(start_row, col, &grid) {
                                        match item {
                                            '.' => {
                                                for (idx, &(item_row, item_col, item_char)) in
                                                    stacked_items.iter().rev().enumerate()
                                                {
                                                    move_item(
                                                        (item_row, item_col),
                                                        (start_row - idx as isize, item_col),
                                                        item_char,
                                                        &mut grid,
                                                    );

                                                    if item_char == '@' {
                                                        // update robot position
                                                        robot_position =
                                                            (start_row - idx as isize, item_col)
                                                    }
                                                }
                                                break;
                                            }
                                            'O' => {
                                                stacked_items.push((start_row, col, 'O'));
                                            }
                                            _ => break,
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                '<' => {
                    if let Some(obj) = get(row, col - 1, &grid) {
                        match obj {
                            '.' => {
                                move_item((row, col), (row, col - 1), '@', &mut grid);
                                robot_position = (row, col - 1);
                            }
                            'O' => {
                                let mut start_col = col - 1;
                                let mut stacked_items =
                                    vec![(row, col, '@'), (row, start_col, 'O')];

                                while start_col >= 0 {
                                    start_col -= 1;

                                    if let Some(item) = get(row, start_col, &grid) {
                                        match item {
                                            '.' => {
                                                for (idx, &(item_row, item_col, item_char)) in
                                                    stacked_items.iter().rev().enumerate()
                                                {
                                                    move_item(
                                                        (item_row, item_col),
                                                        (item_row, start_col + idx as isize),
                                                        item_char,
                                                        &mut grid,
                                                    );

                                                    if item_char == '@' {
                                                        // update robot position
                                                        robot_position =
                                                            (item_row, start_col + idx as isize)
                                                    }
                                                }
                                                break;
                                            }
                                            'O' => {
                                                stacked_items.push((row, start_col, 'O'));
                                            }
                                            _ => break,
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {
                    if let Some(obj) = get(row, col + 1, &grid) {
                        match obj {
                            '.' => {
                                move_item((row, col), (row, col + 1), '@', &mut grid);
                                robot_position = (row, col + 1);
                            }
                            'O' => {
                                let mut start_col = col + 1;
                                let mut stacked_items =
                                    vec![(row, col, '@'), (row, start_col, 'O')];

                                while start_col >= 0 {
                                    start_col += 1;

                                    if let Some(item) = get(row, start_col, &grid) {
                                        match item {
                                            '.' => {
                                                for (idx, &(item_row, item_col, item_char)) in
                                                    stacked_items.iter().rev().enumerate()
                                                {
                                                    move_item(
                                                        (item_row, item_col),
                                                        (item_row, start_col - idx as isize),
                                                        item_char,
                                                        &mut grid,
                                                    );

                                                    if item_char == '@' {
                                                        // update robot position
                                                        robot_position =
                                                            (item_row, start_col - idx as isize)
                                                    }
                                                }

                                                break;
                                            }
                                            'O' => {
                                                stacked_items.push((row, start_col, 'O'));
                                            }
                                            _ => break,
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        let mut gps_coordinates = 0;

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == 'O' {
                    gps_coordinates += 100 * (row as i32) + (col as i32);
                }
            }
        }

        Ok(gps_coordinates)
    }

    pub fn part_2(input: &str) -> Result<i32> {

        let (grid, movements) = load_input_part_2(input);

        for row in &grid {
            for &cell in row {
                print!("{}", cell);
            }
            println!();
        }

        Ok(0)
    }
}

fn move_item(old_pos: (isize, isize), new_pos: (isize, isize), item: char, grid: &mut [Vec<char>]) {
    let (old_row, old_col) = old_pos;
    let (new_row, new_col) = new_pos;

    grid[old_row as usize][old_col as usize] = '.'; // we move the item and leave an empty space
    grid[new_row as usize][new_col as usize] = item;
}

fn get(row: isize, col: isize, grid: &[Vec<char>]) -> Option<char> {
    if row < 0 || col < 0 {
        return None;
    }

    let row = row as usize;
    let col = col as usize;

    if row >= grid.len() || col >= grid[row].len() {
        return None;
    }

    Some(grid[row][col])
}

fn find_robot_position(grid: &[Vec<char>]) -> (isize, isize) {
    for (row, line) in grid.iter().enumerate() {
        if let Some(col) = line.iter().position(|&c| c == '@') {
            return (row as isize, col as isize);
        }
    }

    (0, 0) // Default position if '@' is not found
}

fn load_input_part_2(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let mut input = input.split("\n\n");
    let grid = input
        .next()
        .expect("or invalid input")
        .lines()
        .map(|line| {
            line.chars()
                // apply a mapping function to expand the input
                .flat_map(|ch| match ch {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    _ => vec!['@', '.'],
                })
                .collect()
        })
        .collect();

    let movements = input
        .next()
        .expect("or invalid input")
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    (grid, movements)
}

fn load_input(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let mut input = input.split("\n\n");
    let grid = input
        .next()
        .expect("or invalid input")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let movements = input
        .next()
        .expect("or invalid input")
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    (grid, movements)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        assert_eq!(Solution::part_1(input)?, 10092);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        assert_eq!(Solution::part_2(input)?, 0);

        Ok(())
    }
}

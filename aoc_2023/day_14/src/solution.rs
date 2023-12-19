use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<i64> {
        let mut grid = grid(input);
        let n = grid.len();
        let m = grid[0].len();

        for j in 0..m {
            let line = grid
                .iter()
                .take(n)
                .map(|it| it[j])
                .rev()
                .collect::<String>();

            let slide = slide(&line);

            // put the back the slided string in reversed order
            // maybe make sense to count the tolal right away
            // maybe we don't even need to mutate it at all
        }

        Ok(1)
    }

    // vec![]

    pub fn part_2(input: &str) -> Result<i32> {
        Ok(1)
    }
}

fn grid(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|s| s.trim().chars().collect()).collect()
}

fn slide(line: &str) -> Vec<char> {
    line.split("#")
        .map(slide_rocks)
        .collect::<Vec<_>>()
        .join("#")
        .chars()
        .collect()
}

// in this rock group we will have only soil and rocks nothing else
// the algorithm will count the rocks and generate a string filled with
// soil and the number of rocks
fn slide_rocks<'a>(s: &str) -> String {
    let n = s.len();

    let rocks_len = s.chars().filter(|c| c == &'O').count();
    let soil_len = n - rocks_len;

    let soil_str = ".".repeat(soil_len);
    let rock_str = "O".repeat(rocks_len);

    format!("{}{}", soil_str, rock_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r#"O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
        "#
        .to_string()
    }

    #[test]
    fn test_slide_line_ok() -> Result<()> {
        let scenarios = vec![
            ("O....#....", "....O#....".chars().collect::<Vec<char>>()),
            ("O.OO#....#", ".OOO#....#".chars().collect()),
            (".....##...", ".....##...".chars().collect()),
            (".O.....O#.", "......OO#.".chars().collect()),
            (".......O..", ".........O".chars().collect()),
            ("#OO..#....", "#..OO#....".chars().collect()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = slide(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });

        Ok(())
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 136;
        let input = example();

        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }

    // #[test]
    // fn test_solution_part_2_from_example() -> Result<()> {
    //     let expected = 145;
    //     let input = example();

    //     assert_eq!(Solution::part_2(&input)?, expected);

    //     Ok(())
    // }
}

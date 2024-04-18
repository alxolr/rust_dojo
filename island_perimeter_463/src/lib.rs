pub struct Solution;

#[derive(PartialEq)]
enum Terrain {
    Water,
    Land,
    OutOfBounds,
}

fn get_terain_type(row: isize, col: isize, grid: &[Vec<i32>]) -> Terrain {
    if row < 0 || row >= grid.len() as isize || col < 0 || col >= grid[0].len() as isize {
        Terrain::OutOfBounds
    } else if grid[row as usize][col as usize] == 0 {
        Terrain::Water
    } else {
        Terrain::Land
    }
}

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimiter = 0;

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == 1 {
                    let row = row as isize;
                    let col = col as isize;
                    let local_perimeter = [
                        get_terain_type(row - 1, col, &grid),
                        get_terain_type(row + 1, col, &grid),
                        get_terain_type(row, col - 1, &grid),
                        get_terain_type(row, col + 1, &grid),
                    ]
                    .iter()
                    .filter(|x| x != &&Terrain::Land)
                    .count();

                    perimiter += local_perimeter as i32;
                }
            }
        }

        perimiter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![vec![1]], 4)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::island_perimeter(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

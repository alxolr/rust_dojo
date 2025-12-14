use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<usize> {
        let (shapes, areas) = parse_input(input);

        let count = areas
            .iter()
            .filter(|(area, counts)| {
                let needed_place = counts
                    .iter()
                    .zip(shapes.iter())
                    .map(|(count, shape_area)| (*count) * (*shape_area))
                    .sum::<usize>();

                *area >= needed_place
            })
            .count();

        Ok(count)
    }
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<(usize, Vec<usize>)>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();

    let shapes = &parts[..parts.len() - 1]
        .into_iter()
        .map(|shape| shape.bytes().filter(|bt| bt == &b'#').count())
        .collect::<Vec<_>>();

    let grids = parts[parts.len() - 1];

    let areas = grids
        .lines()
        .map(|line| {
            let mut parts = line.trim().split(": ");
            let area = parts
                .next()
                .unwrap()
                .split("x")
                .flat_map(|it| it.parse::<usize>())
                .fold(1, |mut acc, it| {
                    acc *= it;
                    acc
                });

            let grids = parts
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .flat_map(|it| it.parse())
                .collect();

            (area, grids)
        })
        .collect();

    (shapes.to_vec(), areas)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"0:
        ###
        ##.
        ##.

        1:
        ###
        ##.
        .##

        2:
        .##
        ###
        ##.

        3:
        ##.
        ###
        ##.

        4:
        ###
        #..
        ###

        5:
        ###
        .#.
        ###

        4x4: 0 0 0 0 2 0
        12x5: 1 0 1 0 2 2
        12x5: 1 0 1 0 3 2"#;
        assert_eq!(Solution::part_1(input)?, 0);

        Ok(())
    }
}

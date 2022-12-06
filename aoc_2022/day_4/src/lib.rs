use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    let range_contains =
        |left: (u32, u32), right: (u32, u32)| -> bool { left.0 <= right.0 && left.1 >= right.1 };

    input
        .lines()
        .map(|line| {
            let ranges = line.trim().split(",").collect::<Vec<_>>();
            let [left_range, right_range]: [HashSet<u32>; 2] = ranges
                .into_iter()
                .map(|range| {
                    let values = range.split("-").collect::<Vec<_>>();

                    let left = values.first().unwrap().parse::<u32>().unwrap();
                    let right = values.last().unwrap().parse::<u32>().unwrap();

                    (left..=right).collect::<HashSet<_>>()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            left_range.intersection(&right_range).count() > 0
        })
        .filter(|item| *item)
        .count()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

        assert_eq!(part_1(input), 2);
    }

    #[test]
    fn execute_on_real_input() {
        let input = read_to_string("./input.data").expect("File not found");
        let res = part_1(&input);

        println!("{}", res);
    }
}

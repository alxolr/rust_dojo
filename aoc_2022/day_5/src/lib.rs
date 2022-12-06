use core::num;
use std::collections::VecDeque;

pub fn part_one(input: &str) -> String {
    let [_, moves]: [&str; 2] = input.split("\n\n").collect::<Vec<_>>().try_into().unwrap();

    let mut queues = make_queues_from();
    let things_to_ignore = ["move", "from", "to"];

    moves
        .lines()
        .map(|line| {
            let values: [u32; 3] = line
                .trim()
                .split_whitespace()
                .filter(|word| !things_to_ignore.contains(word))
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            values
        })
        .for_each(|[much, source, target]| {
            let source_idx = source as usize - 1;
            let target_idx = target as usize - 1;

            let mut storage = VecDeque::new();

            for _ in 0..much {
                let value = queues[source_idx].pop_back().unwrap();
                storage.push_back(value);
            }

            for _ in 0..much {
                let value = storage.pop_back().unwrap();
                queues[target_idx].push_back(value);
            }
        });

    for mut queue in queues.into_iter() {
        let value = queue.pop_back().unwrap();

        print!("{}", value);
    }

    "".to_string()
}

fn make_queues_from() -> Vec<VecDeque<char>> {
    vec![
        VecDeque::from_iter(['H', 'T', 'Z', 'D']),
        VecDeque::from_iter(['Q', 'R', 'W', 'T', 'G', 'C', 'S']),
        VecDeque::from_iter(['P', 'B', 'F', 'Q', 'N', 'R', 'C', 'H']),
        VecDeque::from_iter(['L', 'C', 'N', 'F', 'H', 'Z']),
        VecDeque::from_iter(['G', 'L', 'F', 'Q', 'S']),
        VecDeque::from_iter(['V', 'P', 'W', 'Z', 'B', 'R', 'C', 'S']),
        VecDeque::from_iter(['Z', 'F', 'J']),
        VecDeque::from_iter(['D', 'L', 'V', 'Z', 'R', 'H', 'Q']),
        VecDeque::from_iter(['B', 'H', 'G', 'N', 'F', 'Z', 'L', 'D']),
    ]
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn it_works() {
        let input = read_to_string("./input.data").expect("file not found");
        let result = part_one(&input);
        assert_eq!(result, "CMZ".to_string());
    }
}

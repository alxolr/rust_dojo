#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;
type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

struct Solution;
impl Solution {
    fn part_2(lines: &Vec<String>) -> Result<u32> {
        // we have an original set of scratch card
        let mut card_instances = vec![0; lines.len()];
        let mut cards_dictionary: HashMap<u32, usize> = HashMap::new();

        let cards = lines
            .iter()
            .map(|x| x.as_str().try_into())
            .flatten()
            .collect::<Vec<Game>>();

        let mut stack = lines
            .iter()
            .map(|x| x.as_str().try_into())
            .flatten()
            .collect::<VecDeque<Game>>();

        while !stack.is_empty() {
            let current = stack.pop_front().unwrap();
            let current_idx = (current.id - 1) as usize;

            card_instances[current_idx] += 1;

            let entry = cards_dictionary.entry(current.id).or_insert_with(|| {
                let intersect = current
                    .owned
                    .intersection(&current.valid)
                    .map(|x| *x)
                    .collect::<Vec<u32>>();

                intersect.len()
            });

            for card_idx in current_idx + 1..=(*entry + current_idx) {
                stack.push_back(cards[card_idx].clone())
            }
        }


        Ok(card_instances.iter().sum::<u32>())
    }

    fn part_1(lines: &Vec<String>) -> Result<u32> {
        let result = lines
            .iter()
            .map(|x| {
                let game = x.as_str().try_into();
                game
            })
            .flatten()
            .map(|g: Game| {
                let intersect = g
                    .owned
                    .intersection(&g.valid)
                    .map(|x| *x)
                    .collect::<Vec<u32>>();

                let count = intersect.len();

                if count > 0 {
                    2_u32.pow((count - 1) as u32)
                } else {
                    0
                }
            })
            .sum::<u32>();

        Ok(result)
    }
}

#[derive(Debug, PartialEq)]
#[derive(Clone)]
pub struct Game {
    pub id: u32,
    pub valid: HashSet<u32>,
    pub owned: HashSet<u32>,
}

impl TryFrom<&str> for Game {
    type Error = String;

    fn try_from(value: &str) -> core::result::Result<Self, Self::Error> {
        let regex = Regex::new(r"Card\s+(\d+):\s(.*)\s\|\s(.*)").unwrap();
        let maybe_caps = regex.captures(value);

        let numbers = |s: &str| -> Result<HashSet<u32>> {
            let result = s
                .trim()
                .split_whitespace()
                .map(|numbs| numbs.parse::<u32>())
                .flatten()
                .collect::<HashSet<u32>>();

            Ok(result)
        };

        if let Some(caps) = maybe_caps {
            let id = caps[1].parse::<u32>().unwrap();
            let valid: HashSet<u32> = numbers(&caps[2]).unwrap();
            let owned: HashSet<u32> = numbers(&caps[3]).unwrap();

            let game = Game { id, valid, owned };

            Ok(game)
        } else {
            Err("Can't parse the  line into game".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;

    fn example() -> Vec<String> {
        vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_part_1_file_ok() -> Result<()> {
        let lines = load_contents("./data/input")?;

        let result = Solution::part_1(&lines)?;

        println!("result {result}");

        Ok(())
    }

    #[test]
    fn test_part_2_example_ok() -> Result<()> {
        let lines = example();
        let result = Solution::part_2(&lines)?;

        assert_eq!(result, 30);

        Ok(())
    }

    #[test]
    fn test_part_2_file_ok() -> Result<()> {
        let lines = load_contents("./data/input")?;

        let result = Solution::part_2(&lines)?;

        println!("result {result}");

        Ok(())
    }

    #[test]
    fn test_part_1_from_example_ok() -> Result<()> {
        let lines = example();
        let result = Solution::part_1(&lines)?;

        assert_eq!(result, 13);

        Ok(())
    }

    #[test]
    fn test_game_from_string_ok() -> Result<()> {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let game: Game = line.try_into()?;

        assert_eq!(
            game,
            Game {
                id: 1,
                valid: vec![41, 48, 83, 86, 17]
                    .into_iter()
                    .collect::<HashSet<u32>>(),
                owned: vec![83, 86, 6, 31, 17, 9, 48, 53]
                    .into_iter()
                    .collect::<HashSet<u32>>(),
            }
        );

        Ok(())
    }

    fn load_contents(path: &str) -> Result<Vec<String>> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file).lines();

        Ok(buffer.flatten().collect::<Vec<String>>())
    }
}

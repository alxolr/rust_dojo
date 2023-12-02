use core::panic;
use std::{collections::HashMap, fs, path::PathBuf};

use regex::Regex;

struct Solution;

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub data: HashMap<String, u32>,
}
impl Solution {
    pub fn part_1<'a>(lines: &Vec<String>) -> u32 {
        let filter = |g: &Game| {
            let mut is_valid = true;

            for (cube, count) in &g.data {
                let value = match cube.as_str() {
                    "blue" => *count <= 14,
                    "red" => *count <= 12,
                    "green" => *count <= 13,
                    _ => false,
                };
                is_valid = is_valid && value;
            }

            is_valid
        };

        let games = Solution::cube_conundrum(&lines, filter);

        games.iter().map(|g| g.id).sum()
    }

    pub fn part_2(lines: &Vec<String>) -> u32 {
        let filter = |_: &Game| true;

        let games = Solution::cube_conundrum(&lines, filter);

        games.iter().map(|x| {
            let mut prod = 1;
            for val in x.data.values() {
                prod *= *val
            }

            prod
        }).sum()
    }

    pub fn cube_conundrum(lines: &Vec<String>, filter: fn(&Game) -> bool) -> Vec<Game> {
        lines
            .iter()
            .map(|line| Solution::parse_line(line.as_str()))
            .filter(|x| filter(x))
            .collect()
    }

    fn parse_line(line: &str) -> Game {
        let game_id_re = Regex::new(r"Game (\d+):").unwrap();
        let cubes_re = Regex::new(r"(\d+) (\w+)").unwrap();

        let Some(caps) = game_id_re.captures(line) else {
            panic!("Could not parse game id");
        };

        let game_id = &caps[1];
        let line = line.replace(format!("Game {game_id}:").as_str(), "");

        let draws = line
            .split(";")
            .map(|x| x.trim())
            .map(|draw| {
                draw.split(",").map(|cube| {
                    let Some(caps) = cubes_re.captures(cube) else {
                        panic!("could not parse the cube sizes");
                    };

                    (caps[2].to_string(), caps[1].parse::<u32>().unwrap())
                })
            })
            .flatten()
            .fold(HashMap::new(), |mut acc, (k, mut v)| {
                let entry = acc.entry(k).or_insert(v);
                *entry = *entry.max(&mut v);

                acc
            });

        Game {
            id: game_id.parse::<u32>().unwrap(),
            data: draws,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_conundrum_ok() {
        let lines = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];

        assert_eq!(Solution::part_1(&lines), 8);
    }

    #[test]
    fn test_cube_conundrum_part_1_ok() {
        let lines = load_file();

        let result = Solution::part_1(&lines);

        println!("{}", result);
    }

    #[test]
    fn test_cube_conundrum_part_2_ok() {
        let lines = load_file();

        let solution = Solution::part_2(&lines);

        println!("{}", solution);
    }

    fn load_file() -> Vec<String> {
        let file_path = PathBuf::from("./data/input");
        let contents = fs::read_to_string(file_path).expect("could not load the file");

        contents.split("\n").map(|x| x.to_string()).collect()
    }
}

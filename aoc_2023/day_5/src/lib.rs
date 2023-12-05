use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Vec<(u64, u64, u64)>>,
}

fn part_2(almanac_str: String) -> Result<u64> {
    let almanac: Almanac = almanac_str.as_str().try_into()?;

    let result = almanac.process()?;

    Ok(result)
}

impl Almanac {
    fn process(&self) -> Result<u64> {
        let seed_paths = &self
            .seeds
            .par_iter()
            .map(|seed| {
                let path = self.maps.iter().fold(*seed, |destination, ranges| {
                    let mut source: u64 = destination;

                    // find the needed range
                    let maybe_range = ranges.iter().find(|(_, dest_start, rng)| {
                        if destination >= *dest_start && destination < (dest_start + rng) {
                            true
                        } else {
                            false
                        }
                    });

                    if let Some((source_start, dest_start, _)) = maybe_range {
                        let distance = destination - dest_start;

                        source = source_start + distance;
                    }

                    source
                });

                path
            })
            .min()
            .unwrap();

        Ok(*seed_paths)
    }
}

fn match_numbers(haystack: &str) -> Result<Vec<u64>> {
    let numbers_re = Regex::new(r"(\d+)").unwrap();

    let result = numbers_re
        .find_iter(haystack)
        .filter_map(|digits| digits.as_str().parse::<u64>().ok())
        .collect();

    Ok(result)
}

impl TryFrom<&str> for Almanac {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self> {
        let mut seed_line = "";
        let mut maps = vec![];

        let mut stack = vec![];

        for (idx, line) in value.lines().enumerate() {
            if idx == 0 {
                seed_line = line;
                continue;
            }
            if line.is_empty() {
                maps.push(stack.clone());
                stack = vec![];
            } else {
                stack.push(line.trim());
            }
        }

        maps.push(stack.clone()); // don't forget to put the last step in as well

        let maps = maps
            .iter()
            .filter(|it| !it.is_empty())
            .map(|item| {
                item.iter()
                    .map(|l| l.trim())
                    .filter(|line| {
                        let chars: Vec<char> = line.trim().chars().collect();

                        !line.is_empty() && chars.first().unwrap().is_ascii_digit()
                    })
                    .flat_map(match_numbers)
                    .map(|it| {
                        let [destination, source, range]: [u64; 3] = it.try_into().unwrap();

                        (destination, source, range)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let seeds: Vec<u64> = match_numbers(seed_line)?
            .chunks(2)
            .map(|item| {
                let [start, end]: [u64; 2] = item.try_into().unwrap();

                (start..(start + end)).collect::<Vec<u64>>()
            })
            .flatten()
            .collect();

        Ok(Almanac { seeds, maps })
    }
}

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn example() -> String {
        "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
        "
        .to_string()
    }

    fn almanac_example() -> Almanac {
        Almanac {
            seeds: vec![79, 14, 55, 13],
            maps: vec![
                vec![(50, 98, 2), (52, 50, 48)],             // seed to soil
                vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)], // soil to fertilizer
                vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)], // fertilizer to water
                vec![(88, 18, 7), (18, 25, 70)],             // water to light
                vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)], // light to temperature
                vec![(0, 69, 1), (1, 0, 69)],                // temperature to humidity
                vec![(60, 56, 37), (56, 93, 4)],             // humidity to location
            ],
        }
    }

    fn load_contents(path: &str) -> Result<String> {
        Ok(fs::read_to_string(path)?)
    }

    #[test]
    fn test_part_one_from_file_ok() -> Result<()> {
        let example_str = load_contents("./data/input")?;
        let solution = part_2(example_str)?;

        assert_eq!(solution, 24261545);

        Ok(())
    }

    #[test]
    fn test_lowest_location_number_from_example() -> Result<()> {
        let example_str = example();

        let solution = part_2(example_str)?;

        assert_eq!(solution, 46);

        Ok(())
    }

    #[test]
    fn test_fold_seeds_to_maps_ok() -> Result<()> {
        let almanac = almanac_example();

        let expected_paths = 35;

        let paths = almanac.process()?;

        assert_eq!(paths, expected_paths);

        Ok(())
    }

    #[test]
    fn test_create_almanac_from_example_ok() -> Result<()> {
        let almanac_str = example();
        let almanac: Almanac = almanac_str.as_str().try_into()?;

        assert_eq!(almanac, almanac_example());

        Ok(())
    }
}

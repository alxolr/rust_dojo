#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Vec<(u32, u32, u32)>>,
}

impl TryFrom<&str> for Almanac {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self> {
        todo!()
    }
}

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        "
        seeds: 79 14 55 13

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

    #[test]
    fn test_create_almanac_from_example_ok() -> Result<()> {
        let almanac_str = example();

        let almanac: Almanac = almanac_str.as_str().try_into()?;

        assert_eq!(
            almanac,
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
        );

        Ok(())
    }
}

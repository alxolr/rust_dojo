struct Solution;
impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let garbage_types = "MGP";
        let mut total_minutes = 0;

        for garbage_type in garbage_types.chars() {
            let need_truck = garbage.iter().any(|g| g.contains(garbage_type));

            if need_truck {
                // start a car and sum the minutes
                let garbage_units = garbage
                    .iter()
                    .filter(|g| g.contains(garbage_type))
                    .map(|g| g.chars().filter(|c| c == &garbage_type).count() as i32)
                    .sum::<i32>();

                let garbage_minutes = garbage_units * 1;

                let mut left = 0;
                let right = garbage.len();

                let mut travel_minutes = 0;

                while garbage[left..right]
                    .iter()
                    .any(|g| g.contains(garbage_type))
                {
                    if left > 0 { // the car starts at house 0 so we cound only further
                        travel_minutes += travel[left - 1];
                    }

                    left += 1;
                }

                total_minutes += garbage_minutes + travel_minutes;
            }
        }

        total_minutes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_garbage_collection_ok() {
        let scenarios = vec![
            (
                (
                    vec![
                        "G".to_string(),
                        "P".to_string(),
                        "GP".to_string(),
                        "GG".to_string(),
                    ],
                    vec![2, 4, 3],
                ),
                21,
            ),
            (
                (
                    vec!["MMM".to_string(), "PGM".to_string(), "GP".to_string()],
                    vec![3, 10],
                ),
                37,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((garbage, travel), expected))| {
                let result = Solution::garbage_collection(garbage, travel);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

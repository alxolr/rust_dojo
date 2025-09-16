use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let mut cannot_communicate = HashSet::new();

        for friendship in friendships {
            let first = friendship[0] - 1;
            let second = friendship[1] - 1;

            let first_language = languages[first as usize].iter().collect::<HashSet<_>>();
            let second_languages = languages[second as usize].iter().collect::<HashSet<_>>();

            if first_language.intersection(&second_languages).count() == 0 {
                cannot_communicate.insert(first);
                cannot_communicate.insert(second);
            }
        }

        let mut max_language_count = 0;
        let mut language_count = HashMap::new();

        for person in cannot_communicate.iter() {
            for language in languages[*person as usize].iter() {
                let entry = language_count.entry(language).or_insert(0);
                *entry += 1;

                max_language_count = max_language_count.max(*entry);
            }
        }

        cannot_communicate.len() as i32 - max_language_count
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(
            (
                2,
                vec![vec![1], vec![2], vec![1, 2]],
                vec![vec![1, 2], vec![1, 3], vec![2, 3]],
            ),
            1,
        )];

        scenarios.into_iter().enumerate().for_each(
            |(idx, ((n, languages, friendships), expected))| {
                let result = Solution::minimum_teachings(n, languages, friendships);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            },
        );
    }
}

use std::{cmp::Ordering, collections::HashMap};

use crate::error::Result;

pub struct Solution;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum HandType {
    FiveKind = 6,
    FourKind = 5,
    FullHouse = 4,
    ThreeKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn card_strength(cards: &str) -> HashMap<char, u64> {
    let strength = 0..cards.len();
    cards
        .chars()
        .zip(strength)
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k, v as u64);
            acc
        })
}

impl Solution {
    pub fn process(
        input: &str,
        strategy: fn(&str) -> Result<HandType>,
        cards_order: &str,
    ) -> Result<u64> {
        let mut hands = parse_input(input)?;
        let card_strength = card_strength(cards_order);

        hands.sort_by(|a, b| {
            let a_type = strategy(&a.0).unwrap();
            let b_type = strategy(&b.0).unwrap();

            if a_type > b_type {
                Ordering::Greater
            } else if a_type < b_type {
                Ordering::Less
            } else {
                let a_chs: Vec<char> = a.0.chars().collect();
                let b_chs: Vec<char> = b.0.chars().collect();

                let mut ord = Ordering::Equal;

                for idx in 0..5 {
                    let a_card = card_strength.get(&a_chs[idx]).unwrap();
                    let b_card = card_strength.get(&b_chs[idx]).unwrap();

                    match a_card.cmp(b_card) {
                        Ordering::Less => {
                            ord = Ordering::Less;
                            break;
                        }
                        Ordering::Equal => continue,
                        Ordering::Greater => {
                            ord = Ordering::Greater;
                            break;
                        }
                    }
                }

                ord
            }
        });

        let result = hands
            .iter()
            .enumerate()
            .map(|(idx, (_, bid))| (idx + 1) as u64 * (*bid))
            .sum::<u64>();

        Ok(result)
    }

    pub fn part_2(input: &str) -> Result<u64> {
        Solution::process(input, hand_type_with_jokers, "J23456789TQKA")
    }

    pub fn part_1(input: &str) -> Result<u64> {
        Solution::process(input, hand_type, "23456789TJQKA")
    }
}

fn parse_input(input: &str) -> Result<Vec<(String, u64)>> {
    let result = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let parsed = l.split_whitespace().collect::<Vec<_>>();

            let [hand, bid]: [&str; 2] = parsed.try_into().unwrap();

            (hand.to_string(), bid.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    Ok(result)
}

fn hand_type_with_jokers(hand: &str) -> Result<HandType> {
    let mut card_counts = hand.chars().fold(HashMap::new(), |mut acc, item| {
        let entry = acc.entry(item).or_insert(0);
        *entry += 1;

        acc
    });

    if card_counts.contains_key(&'J') && card_counts.len() > 1 {
        // move the jocker to the highest count number
        let (_, joker_val) = card_counts.remove_entry(&'J').unwrap();

        let (max, _) = card_counts
            .iter()
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .unwrap();

        card_counts.entry(*max).and_modify(|val| *val += joker_val);
    }

    let result = get_hand_type(card_counts);

    Ok(result)
}

fn hand_type(hand: &str) -> Result<HandType> {
    let card_counts = hand.chars().fold(HashMap::new(), |mut acc, item| {
        let entry = acc.entry(item).or_insert(0);
        *entry += 1;

        acc
    });

    let result = get_hand_type(card_counts);

    Ok(result)
}

fn get_hand_type(card_counts: HashMap<char, i32>) -> HandType {
    match card_counts.len() {
        5 => HandType::HighCard,
        4 => HandType::OnePair,
        3 => {
            if card_counts.values().any(|x| x == &3) {
                HandType::ThreeKind
            } else {
                HandType::TwoPair
            }
        }
        2 => {
            if card_counts.values().all(|x| x >= &2) {
                HandType::FullHouse
            } else {
                HandType::FourKind
            }
        }
        1 => HandType::FiveKind,
        _ => panic!("There can't be more than 5 cards"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r"32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        "
        .to_string()
    }

    #[test]
    fn test_hand_type_with_jokers_ok() -> Result<()> {
        let scenarios = vec![
            ("23J56", HandType::OnePair), // we count the cards and there are only one each
            ("JJJJJ", HandType::FiveKind), // we count the cards and found that one card has two
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = hand_type_with_jokers(input).unwrap();

                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });

        Ok(())
    }

    #[test]
    fn test_figure_hand_type_ok() -> Result<()> {
        let scenarios = vec![
            ("23456", HandType::HighCard), // we count the cards and there are only one each
            ("23452", HandType::OnePair),  // we count the cards and found that one card has two
            ("23352", HandType::TwoPair),  // we count the cards and found that one card has two
            ("33352", HandType::ThreeKind), // we count the cards and found that one card has two
            ("33322", HandType::FullHouse), // we count the cards and found that one card has two
            ("AAAA3", HandType::FourKind),
            ("33333", HandType::FiveKind), // we count the cards and found that one card has two
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = hand_type(input).unwrap();

                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });

        Ok(())
    }

    #[test]
    fn test_parse_input_ok() -> Result<()> {
        let input = example();
        let expected_result = vec![
            ("32T3K".to_string(), 765),
            ("T55J5".to_string(), 684),
            ("KK677".to_string(), 28),
            ("KTJJT".to_string(), 220),
            ("QQQJA".to_string(), 483),
        ];
        let result = parse_input(&input)?;

        assert_eq!(result, expected_result);

        Ok(())
    }

    #[test]
    fn test_solution_part_2_from_example() -> Result<()> {
        let expected = 5905;
        let input = example();
        assert_eq!(Solution::part_2(&input)?, expected);

        Ok(())
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 6440;
        let input = example();
        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }
}

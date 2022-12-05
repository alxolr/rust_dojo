use std::collections::{HashMap, HashSet};

pub fn part_1(rucksacs: &str) -> u32 {
    // let priorities = ['a'..'z'].iter().collect::<Vec<_>>();
    let mut priorities: HashMap<char, u32> = HashMap::new();

    for (idx, ch) in ('a'..='z').into_iter().enumerate() {
        priorities.insert(ch, idx as u32 + 1);
    }

    for (idx, ch) in ('A'..='Z').into_iter().enumerate() {
        priorities.insert(ch, idx as u32 + 27);
    }

    rucksacs
        .lines()
        .map(|line| {
            let line = line.trim();
            let mid = line.len() / 2;
            let items = line.chars().clone();
            let other_side = line.chars().clone();

            let first_sack = items.into_iter().take(mid).collect::<HashSet<char>>();

            let second_sack = other_side.into_iter().skip(mid).collect::<HashSet<char>>();

            let value = first_sack.intersection(&second_sack).collect::<Vec<_>>();

            priorities.get(value[0]).unwrap()
        })
        .sum::<u32>()
}

pub fn part_2(input: &str) -> u32 {
    let mut priorities: HashMap<char, u32> = HashMap::new();

    let range = ('a'..='z').chain('A'..='Z');
    let all_letters = range.clone().collect::<HashSet<_>>();

    let mut idx = 1;
    for ch in range {
        priorities.insert(ch, idx);
        idx += 1;
    }

    let items = input.lines().collect::<Vec<_>>();

    items
        .chunks(3)
        .map(|lines| {
            let item = lines.iter().fold(all_letters.clone(), |acc, curr_line| {
                let hash_line = curr_line.chars().collect::<HashSet<_>>();

                let intersection = acc.intersection(&hash_line);

                intersection.cloned().collect::<HashSet<_>>()
            });

            let val = item.iter().take(1).collect::<Vec<_>>();

            priorities.get(&val[0]).unwrap()
        })
        .sum()

    // items.chunks(3).map(|lines| {
    // let first_sack = first.chars().collect::<HashSet<char>>();
    // let second_sack = second.chars().collect::<HashSet<char>>();
    // let third_sack = third.chars().collect::<HashSet<char>>();

    // let value = first_sack.intersection(&second_sack)
    // })
    // // .()
    // .map(|line| {
    //     let line = line.trim();
    //     let mid = line.len() / 2;
    //     let items = line.chars().clone();
    //     let other_side = line.chars().clone();

    //     let first_sack = items.into_iter().take(mid).collect::<HashSet<char>>();

    //     let second_sack = other_side.into_iter().skip(mid).collect::<HashSet<char>>();

    //     let value = first_sack.intersection(&second_sack).collect::<Vec<_>>();

    //     priorities.get(value[0]).unwrap()
    // })
    // .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn it_works() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(part_1(input), 157);
    }

    #[test]
    fn do_test_real_stuff() {
        let input = read_to_string("./input.data").expect("File not found");
        println!("{}", part_1(&input));
    }

    #[test]
    fn do_test_part_2() {
        let input = read_to_string("./input.data").expect("File not found");

        println!("{}", part_2(&input));
    }
}

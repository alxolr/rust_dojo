use core::num;
use std::{cmp::Ordering, fs, path::PathBuf};

fn main() {
    let result = part_two();

    println!("result {}", result);
}

fn process_line(line: &str, numbers: &Vec<(String, String)>) -> i32 {
    // the idea will be different

    // we will need to process the line in two iterators
    // one from start to end
    // and the second in reversed order

    let mut line = line;
    let mut stack = String::new();
    let mut first: String = String::from("");

    for ch in line.chars() {
        if ch.is_alphabetic() {
            stack.push(ch);

            if stack.len() > 2 {
                for (alphabetic, numeric) in numbers {
                    if stack.contains(alphabetic) {
                        first = numeric.clone();
                        break;
                    }
                }

                if first != "".to_string() {
                    break;
                }
            }
        } else {
            first = ch.to_string();
            break;
        }
    }

    let mut last: String = "".to_string();

    for ch in line.chars().rev() {
        if ch.is_alphabetic() {
            stack.push(ch);

            if stack.len() > 2 {
                for (alphabetic, numeric) in numbers {
                    let reverse: String = stack.chars().rev().collect();
                    if reverse.contains(alphabetic) {
                        last = numeric.clone();
                        break;
                    }
                }

                if last != "".to_string() {
                    break;
                }
            }
        } else {
            last = ch.to_string();
            break;
        }
    }

    let number: String = vec![first, last].into_iter().collect();
    let number = number.parse::<i32>().unwrap_or(0);

    number

    // number
}

fn part_two() -> i32 {
    // let file_path = PathBuf::from("./data/simple.txt");
    let file_path = PathBuf::from("./data/input.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut p2: i32 = 0;
    for line in contents.split("\n") {
        let (first, last) =
            line.chars()
                .enumerate()
                .fold((None, None), |(mut first, mut last), (index, c)| {
                    if c.is_ascii_digit() {
                        let number = c as u32 - '0' as u32;
                        first = first
                            .filter(|f: &(_, _)| f.1 < index)
                            .or(Some((number, index)));
                        last = last
                            .filter(|f: &(_, _)| f.1 > index)
                            .or(Some((number, index)));
                    }
                    (first, last)
                });
        const NUMS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let first_indices = NUMS.map(|n| line.find(n));
        let last_indices: [_; 9] =
            std::array::from_fn(|i| first_indices[i].and_then(|_| line.rfind(NUMS[i])));
        let (first, last) = (first_indices.into_iter().zip(last_indices.into_iter()))
            .zip(1..)
            .fold(
                (first, last),
                |(mut first, mut last), ((first_match_index, last_match_index), number)| {
                    if let Some(index) = first_match_index {
                        first = first
                            .filter(|f: &(_, _)| f.1 < index)
                            .or(Some((number, index)));
                    }
                    if let Some(index) = last_match_index {
                        last = last
                            .filter(|f: &(_, _)| f.1 > index)
                            .or(Some((number, index)));
                    }
                    (first, last)
                },
            );

        let first2 = first.expect("test");
        let last2 = last.expect("test");

        let num2 = (first2.0 * 10) + last2.0;

        p2 += num2 as i32;
    }

    p2
}

fn part_one() -> i32 {
    let file_path = PathBuf::from("./data/input.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let result: i32 = contents
        .split("\n")
        .map(|x| {
            let codes = x.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();

            let first = codes[0];
            let last = codes[codes.len() - 1];

            let number: String = vec![first, last].iter().collect();
            let number = number.parse::<i32>().unwrap_or(0);

            number
        })
        .sum();
    result
}

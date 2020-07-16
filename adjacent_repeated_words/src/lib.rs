pub fn count_adjacent_pairs(search_string: &str) -> usize {
    if search_string.len() == 0 {
        return 0;
    } else {
        let lower_cased = search_string.to_lowercase();
        let splitted_words=  lower_cased.split_whitespace().collect::<Vec<_>>();
        let mut prev = "".to_string();

        return splitted_words
            .windows(2)
            .fold(0, |mut acc, item| {
                if !(join_str(item) == prev) {
                    prev = join_str(item);
                    if item[0] == item[1] {
                        acc += 1;
                    }
                }

                acc
            }) as usize;
    }
}

fn join_str(item: &[&str]) -> String {
    format!("{}{}", item[0], item[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            count_adjacent_pairs(&""),
            0,
            "An empty string should return 0"
        );
        assert_eq!(
            count_adjacent_pairs(&"orange Orange kiwi pineapple apple"),
            1,
            "Case should be ignored"
        );
        assert_eq!(count_adjacent_pairs(&"banana banana banana"), 1, "Once a word has been paired, it is ignored. countAdjacentPairs(\"banana banana banana\")");
        assert_eq!(count_adjacent_pairs(&"banana banana banana terracotta banana terracotta terracotta pie!"), 2, "Once a word has been paired, it is ignored. Grab all pairs. countAdjacentPairs(\"banana banana banana terracotta banana terracotta terracotta pie!\")");
        assert_eq!(
            count_adjacent_pairs(&"pineapple apple"),
            0,
            "A pineapple is not an apple. countAdjacentPairs(\"pineapple apple\")"
        );

        assert_eq!(count_adjacent_pairs("print print setattr setattr setattr bytes bytes bytes zip zip slice dict dict bytes bytes bytes setattr setattr setattr dir super"), 7);
    }
}

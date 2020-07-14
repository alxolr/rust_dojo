use std::collections::HashMap;

fn string_letter_count(s: &str) -> String {
    let counter: HashMap<char, i32> = HashMap::new();

    let mut touples = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(counter, |mut acc, ch| {
            *acc.entry(ch).or_insert(0) += 1;

            acc
        })
        .into_iter()
        .collect::<Vec<(char, i32)>>();

    touples.sort_by(|x, y| x.0.cmp(&y.0));

    touples
        .iter()
        .fold("".to_string(), |acc, it| format!("{}{}{}", acc, it.1, it.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            string_letter_count(&"The quick brown fox jumps over the lazy dog."),
            "1a1b1c1d3e1f1g2h1i1j1k1l1m1n4o1p1q2r1s2t2u1v1w1x1y1z"
        );
        assert_eq!(
            string_letter_count(&"The time you enjoy wasting is not wasted time."),
            "2a1d5e1g1h4i1j2m3n3o3s6t1u2w2y"
        );
        assert_eq!(string_letter_count(&"./4592#{}()"), "");
    }
}

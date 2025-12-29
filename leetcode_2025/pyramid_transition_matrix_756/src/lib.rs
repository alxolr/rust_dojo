use std::collections::{HashSet, VecDeque};

impl Solution {
    fn backtrack(
        idx: usize,
        current_path: &mut Vec<char>,
        solution_pool: &Vec<Vec<char>>,
        results: &mut Vec<String>,
    ) {
        if current_path.len() == solution_pool.len() {
            // it's a solution
            results.push(current_path.iter().collect::<String>());
            return;
        }

        if idx < solution_pool.len() {
            for ch in solution_pool[idx].iter() {
                current_path.push(*ch);
                Self::backtrack(idx + 1, current_path, solution_pool, results);
                current_path.pop();
            }
        }
    }

    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_front((bottom.clone(), bottom.len() - 1));

        while let Some((base, level)) = queue.pop_front() {
            // if the level is one then we need to check if there is one allowed exit
            if level == 1 {
                for top in 'A'..='F' {
                    let pyramid = format!("{base}{}", top);
                    if allowed.contains(&pyramid) {
                        return true;
                    }
                }
            }

            let base_as_chs = base.chars().collect::<Vec<_>>();
            let combos: Vec<Vec<char>> = base_as_chs
                .windows(2)
                .map(|two| {
                    let bottom = two.iter().collect::<String>();

                    ('A'..='F')
                        .filter(|top| allowed.contains(&format!("{bottom}{top}")))
                        .collect::<Vec<_>>()
                })
                .collect();

            let mut new_bases = vec![];
            Solution::backtrack(0, &mut vec![], &combos, &mut new_bases);

            new_bases.iter().for_each(|base| {
                if !visited.contains(base) {
                    queue.push_back((base.clone(), level - 1));
                    visited.insert(base.clone());
                }
            })
            // generate all the possible bases using backtracking
        }

        false
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pyramid_transition_ok() {
        let scenarios = vec![
            (
                (
                    "BCD".to_string(),
                    vec![
                        "BCC".to_string(),
                        "CDE".to_string(),
                        "CEA".to_string(),
                        "FFF".to_string(),
                    ],
                ),
                true,
            ),
            (
                (
                    "AAAA".to_string(),
                    vec![
                        "AAB".to_string(),
                        "AAC".to_string(),
                        "BCD".to_string(),
                        "BBE".to_string(),
                        "DEF".to_string(),
                    ],
                ),
                false,
            ),
            (
                (
                    "BBDCDA".to_string(),
                    vec![
                        "CAB".to_string(),
                        "ACB".to_string(),
                        "ACA".to_string(),
                        "AAA".to_string(),
                        "AAC".to_string(),
                        "AAB".to_string(),
                        "CDB".to_string(),
                        "BCA".to_string(),
                        "CBB".to_string(),
                        "BCC".to_string(),
                        "BAB".to_string(),
                        "BAC".to_string(),
                        "BAA".to_string(),
                        "CCD".to_string(),
                        "CAA".to_string(),
                        "CCA".to_string(),
                        "CCC".to_string(),
                        "CCB".to_string(),
                        "DAD".to_string(),
                        "DAA".to_string(),
                        "DAC".to_string(),
                        "ACD".to_string(),
                        "DCB".to_string(),
                        "DCC".to_string(),
                        "DCA".to_string(),
                        "CAD".to_string(),
                        "ACC".to_string(),
                        "ABA".to_string(),
                        "ABB".to_string(),
                        "ABD".to_string(),
                        "BBD".to_string(),
                        "BBB".to_string(),
                        "BBA".to_string(),
                        "ADD".to_string(),
                        "ADB".to_string(),
                        "ADC".to_string(),
                        "DDC".to_string(),
                        "DDB".to_string(),
                        "DDA".to_string(),
                        "DDD".to_string(),
                        "CDD".to_string(),
                        "CBC".to_string(),
                        "CBA".to_string(),
                        "CDA".to_string(),
                        "CBD".to_string(),
                        "CDC".to_string(),
                        "DBC".to_string(),
                        "DBD".to_string(),
                        "BDA".to_string(),
                    ],
                ),
                true,
            ),
        ];

        for (idx, scenario) in scenarios.into_iter().enumerate() {
            assert_eq!(
                Solution::pyramid_transition(scenario.0.0, scenario.0.1),
                scenario.1
            );

            println!("scenario {} ok", idx + 1);
        }
    }
}

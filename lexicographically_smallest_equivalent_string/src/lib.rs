use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let groups =
            s1.chars()
                .zip(s2.chars())
                .fold(vec![], |mut acc: Vec<HashSet<char>>, (l, r)| {
                    let groups = acc
                        .iter()
                        .enumerate()
                        .filter(|(_, it)| it.contains(&l) || it.contains(&r))
                        .map(|(idx, _)| idx)
                        .collect::<Vec<_>>();

                    match groups.len() {
                        0 => {
                            let set = vec![l, r].into_iter().collect::<HashSet<_>>();
                            acc.push(set);

                            acc
                        }
                        1 => {
                            acc[groups[0]].insert(l);
                            acc[groups[0]].insert(r);

                            acc
                        }
                        _ => {
                            let union_set =
                                groups.iter().fold(HashSet::<char>::new(), |res, id| {
                                    let union_iter = res.union(&acc[*id]);

                                    union_iter.map(|c| *c).collect::<HashSet<char>>()
                                });

                            let mut cleaned_acc = acc
                                .into_iter()
                                .enumerate()
                                .filter(|(idx, _)| !groups.contains(idx))
                                .map(|(_, value)| value)
                                .collect::<Vec<_>>();

                            cleaned_acc.push(union_set);

                            cleaned_acc
                        }
                    }
                });

        let groups = groups
            .iter()
            .map(|x| {
                let mut v = x.iter().collect::<Vec<_>>();
                v.sort();
                v
            })
            .collect::<Vec<_>>();

        base_str
            .chars()
            .map(|ch| {
                for group in groups.iter() {
                    if group.contains(&&ch) {
                        let first = group[0];

                        return *first;
                    }
                }

                ch
            })
            .collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (
                    "parker".to_string(),
                    "morris".to_string(),
                    "parser".to_string(),
                ),
                "makkek".to_string(),
            ),
            (
                (
                    "cgokcgerolkgksgbhgmaaealacnsshofjinidiigbjerdnkolc".to_string(),
                    "rjjlkbmnprkslilqmbnlasardrossiogrcboomrbcmgmglsrsj".to_string(),
                    "bxbwjlbdazfejdsaacsjgrlxqhiddwaeguxhqoupicyzfeupcn".to_string(),
                ),
                "axawaaaaazaaaaaaaaaaaaaxaaaaawaaauxaaauaaayzaauaaa".to_string(),
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((s1, s2, base_str), expected))| {
                let result = Solution::smallest_equivalent_string(s1, s2, base_str);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

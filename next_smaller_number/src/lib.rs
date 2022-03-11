fn next_smaller_number(n: u64) -> Option<u64> {
    let chars = n.to_string().chars().collect::<Vec<_>>();

    if !has_solution(&chars) {
        return None;
    }

    let chars_clone = chars.clone();
    let chars_len = chars.len();
    let mut local_solutions = vec![];

    if !has_solution(&chars) {
        return None;
    }

    for ch in chars.into_iter().rev() {
        local_solutions.push(ch);

        if local_solutions.len() > 1 {
            let rev_solution = local_solutions
                .clone()
                .into_iter()
                .rev()
                .collect::<Vec<_>>();

            let maybe_solution = next_smaller_number_small(&rev_solution);
            if maybe_solution.is_some() {
                let result = maybe_solution.unwrap();

                if result.len() == chars_len {
                    return Some(result.iter().collect::<String>().parse::<u64>().unwrap());
                } else {
                    let mut first_part = chars_clone
                        .into_iter()
                        .take(chars_len - result.len())
                        .collect::<Vec<_>>();
                    first_part.extend(result);

                    let total = first_part.iter().collect::<String>();

                    return Some(total.parse::<u64>().unwrap());
                }
            }
        }
    }

    None
}

fn next_smaller_number_small(vec: &Vec<char>) -> Option<Vec<char>> {
    let n = vec.iter().collect::<String>().parse::<u64>().unwrap();

    let mut solutions = generate_combos(&vec)
        .into_iter()
        .filter(|x| x[0] != '0')
        .map(|x| x.iter().collect::<String>())
        .map(|x| x.parse::<u64>().unwrap())
        .filter(|x| *x < n)
        .collect::<Vec<_>>();

    solutions.sort();

    if solutions.len() > 0 {
        let last = solutions.last().unwrap();
        let result: Vec<char> = last.to_string().chars().collect::<Vec<_>>();

        return Some(result);
    }

    None
}

fn generate_combos(vec: &Vec<char>) -> Vec<Vec<char>> {
    let mut solutions = vec![];

    if vec.len() == 1 {
        solutions.push(vec.clone());
    }

    for idx in 0..vec.len() {
        let mut cloned_vec = vec.clone();
        let value_as_vec = vec![vec[idx]];
        cloned_vec.remove(idx);

        if cloned_vec.len() > 0 {
            let combos = generate_combos(&cloned_vec);

            let joined_combos = combos
                .into_iter()
                .map(|combo| {
                    let mut cloned = value_as_vec.clone();
                    cloned.extend(combo);

                    cloned
                })
                .collect::<Vec<_>>();

            joined_combos.into_iter().for_each(|combo| {
                solutions.push(combo);
            });
        }
    }

    solutions
}

fn has_solution(vec: &Vec<char>) -> bool {
    let mut prev = vec[0];

    if vec.into_iter().filter(|x| *x != &prev).count() == 0 {
        return false;
    }

    for idx in 0..vec.len() {
        let curr = vec[idx];
        if curr == '0' {
            continue;
        }
        if curr >= prev {
            prev = curr;
        } else {
            return true;
        }
    }

    let zero_pos = vec.iter().position(|x| x == &'0');
    
    match zero_pos {
        Some(pos) => {
            let len = vec.len() / 2;
            if pos >= len {
                return true;
            }
        }
        None => {}
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_solution_for_sorted() {
        assert_eq!(
            false,
            has_solution(&vec!['1', '0', '2', '3', '4', '5', '6', '7', '8', '9'])
        );
    }

    #[test]
    fn example() {
        assert_eq!(Some(12), next_smaller_number(21));
        assert_eq!(Some(790), next_smaller_number(907));
        assert_eq!(Some(513), next_smaller_number(531));
        assert_eq!(None, next_smaller_number(1027));
        assert_eq!(Some(1072), next_smaller_number(1207));
        assert_eq!(Some(414), next_smaller_number(441));
        assert_eq!(Some(153), next_smaller_number(315));
        assert_eq!(Some(1234567890), next_smaller_number(1234567908));
        assert_eq!(
            Some(17098580789551117865),
            next_smaller_number(17098580789551118567)
        );

        assert_eq!(None, next_smaller_number(11112222333344445566));
    }
}

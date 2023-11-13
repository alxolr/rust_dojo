fn wave_sort(xs: &mut [i32]) {
    xs.sort();

    let smaller_iter = xs.iter().take(xs.len() / 2);
    let bigger_iter = xs.iter().skip(xs.len() / 2);

    let zipped: Vec<i32> = smaller_iter
        .zip(bigger_iter)
        .map(|x| vec![x.1, x.0])
        .flatten()
        .map(|x| x.clone())
        .collect();

    for (idx, item) in zipped.iter().enumerate() {
        xs[idx] = *item;
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;

    fn is_wave_sorted(xs: &[i32]) -> bool {
        let mut is_bigger = false;

        for (idx, pair) in xs.windows(2).enumerate() {
            let left = pair[0];
            let right: i32 = pair[1];

            if left == right {
                return false;
            }

            if idx == 0 {
                if left < right {
                    is_bigger = false;
                } else {
                    is_bigger = true;
                }
            } else {
                if is_bigger {
                    if left < right {
                        return false;
                    }
                } else {
                    if left > right {
                        return false;
                    }
                }
            }

            is_bigger = !is_bigger;
        }

        true
    }

    #[test]
    fn test_is_wave_sorted() {
        let scenarios = vec![
            (vec![1, 2, 3], false),
            (vec![2, 2], false),
            (vec![3, 2, 1], false),
            (vec![3, 4, 3, 3], false),
            (vec![3], true),
        ];

        for (xs, exp_result) in scenarios.into_iter() {
            assert_eq!(is_wave_sorted(&xs), exp_result);
        }
    }

    #[test]
    fn test_scenarios() {
        let scenarios = vec![(vec![1, 2, 3], vec![1, 3, 2])];

        for (mut xs, exp_result) in scenarios.into_iter() {
            wave_sort(&mut xs);
            assert_eq!(xs, exp_result);
        }
    }
}

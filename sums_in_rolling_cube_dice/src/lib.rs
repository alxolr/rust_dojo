use float_eq::float_eq;

fn rolldice_sum_prob(sum: i32, dice_amount: i32) -> f64 {
    let combos = 6_f64.powf(dice_amount as f64);
    // will return all cobinations possible to crate using dices the sum amount
    let solutions = roll_dices_sum(sum, dice_amount).len();

    solutions as f64 / combos
}

pub fn roll_dices_sum(sum: i32, dice_number: i32) -> Vec<Vec<i32>> {
    let mut solutions = vec![];

    if sum > 6 && dice_number == 1 {
        return solutions;
    }

    if dice_number == 1 && sum <= 6 && sum >= 1 {
        solutions.push(vec![sum]);
    } else {
        for cube in (1..=6).rev() {
            if sum - cube > 0 {
                let local = roll_dices_sum(sum - cube, dice_number - 1)
                    .into_iter()
                    .map(|mut vec| {
                        vec.push(cube);
                        vec
                    })
                    .collect::<Vec<_>>();

                solutions.extend(local);
            }
        }
    }

    solutions
}

mod tests {
    use super::*;

    fn assert_float_equals(actual: f64, expected: f64, merr: f64) {
        let res =
            float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
        assert!(
            res,
            "Expected value must be near: {:e} but was:{:e}",
            expected, actual
        );
    }

    #[test]
    fn returns_expected() {
        assert_float_equals(rolldice_sum_prob(11, 2), 0.055555555555, 1e-10);
        assert_float_equals(rolldice_sum_prob(8, 2), 0.13888888889, 1e-10);
        assert_float_equals(rolldice_sum_prob(8, 3), 0.0972222222222, 1e-10);
        assert_float_equals(rolldice_sum_prob(22, 3), 0.0, 1e-10);
    }

    #[test]
    fn test_ways_to_make_sum() {
        let scenarios = vec![
            ((3, 1), vec![vec![3]]),
            ((8, 1), vec![]),
            (
                (6, 2),
                vec![vec![1, 5], vec![2, 4], vec![3, 3], vec![4, 2], vec![5, 1]],
            ),
        ];

        scenarios
            .into_iter()
            .for_each(|((sum, dice_count), expected)| {
                let result = roll_dices_sum(sum, dice_count);

                println!("{:?}", result);
                assert_eq!(result, expected)
            });
    }
}

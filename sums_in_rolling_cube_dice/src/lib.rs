use std::collections::{HashMap, HashSet};

use float_eq::float_eq;

fn fact(n: i32) -> i32 {
    if n == 1 {
        1
    } else {
        n * fact(n - 1)
    }
}

fn compute_permutations(vec: &Vec<i32>) -> i32 {
    let mut hashmap: HashMap<i32, usize> = HashMap::new();
    let len = vec.len();

    for key in vec.iter() {
        let entry = hashmap.entry(*key).or_insert(0);
        *entry += 1;
    }

    let vec: Vec<usize> = hashmap.into_values().collect();
    let denominator = vec.iter().fold(1, |mut acc, item| {
        acc *= fact(*item as i32);
        acc
    });

    fact(len as i32) / denominator
}

fn subset_sum(capacity: usize, sum: i32) -> Vec<Vec<i32>> {
    let mut solutions = vec![];
    let mut solution = vec![0; capacity];
    let avg = sum / capacity as i32;
    solution.fill(avg);

    loop {
        let local_sum: i32 = solution.iter().sum();
        if local_sum == sum {
            solutions.push(solution.clone())
        }

        let count = solution.iter().filter(|x| **x == 1).count();
        if count == capacity {
            break;
        }

        // maybe apply some divide and conquerr technique 

        solution = substract_from_dice_array(solution);  
    }

    solutions
}

fn rolldice_sum_prob(sum: i32, dice_amount: i32) -> f64 {
    let combos = 6_f64.powf(dice_amount as f64);
    let solutions: Vec<Vec<i32>> = subset_sum(dice_amount as usize, sum);

    let mut local_combos = 0;

    let mut h_set: HashSet<Vec<i32>> = HashSet::new();

    for mut solution in solutions.into_iter() {
        solution.sort();

        if !h_set.contains(&solution) {
            local_combos += compute_permutations(&solution);
            h_set.insert(solution);
        }
    }

    local_combos as f64 / combos
}

fn substract_from_dice_array(mut dice_combo: Vec<i32>) -> Vec<i32> {
    let len = dice_combo.len();
    let mut index = len - 1;

    loop {
        if dice_combo[index] >= 2 {
            dice_combo[index] = dice_combo[index] - 1;
            for item in index + 1..len {
                dice_combo[item] = 6;
            }

            break;
        } else {
            index -= 1;
        }
    }

    dice_combo
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
        assert_float_equals(rolldice_sum_prob(100, 100), 0.000001, 1e-10);


    }

    #[test]
    fn calculate_permutations_for_repetition() {
        let scenarios = vec![
            (vec![1, 2, 3], 6),
            (vec![1, 2, 2], 3),
            (vec![1, 2, 2, 4, 4], 30),
            (vec![4, 4], 1),
        ];

        scenarios.iter().for_each(|(scenario, expected)| {
            let result = compute_permutations(scenario);

            assert_eq!(&result, expected)
        });
    }

    #[test]
    fn test_subset_sum_with_given_size() {
        let scenarios = vec![((3_usize, 3), vec![vec![1, 1, 1i32]])];

        scenarios.iter().for_each(|(scenario, expected)| {
            let (capacity, sum) = scenario;
            let result = subset_sum(*capacity, *sum);

            assert_eq!(&result, expected)
        });
    }

    #[test]
    fn test_substract_one_from_dice_array() {
        let scenarios = vec![
            (vec![6, 6, 6], vec![6, 6, 5]),
            (vec![6, 6, 1], vec![6, 5, 6]),
            (vec![6, 5, 2], vec![6, 5, 1]),
            (vec![6, 1, 1], vec![5, 6, 6]),
            (vec![2, 3, 3], vec![2, 3, 2]),
            (vec![1, 1, 2], vec![1, 1, 1]),
        ];
        scenarios.into_iter().for_each(|(scenario, expected)| {
            let result = substract_from_dice_array(scenario);

            assert_eq!(result, expected)
        });
    }
}

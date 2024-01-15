use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let player_stats: HashMap<i32, (i32, i32)> =
            matches.iter().fold(HashMap::new(), |mut acc, game| {
                let winner = game[0];
                let looser = game[1];
                let winner_entry = acc.entry(winner).or_insert((0, 0));
                (*winner_entry).0 += 1;

                let loser_entry = acc.entry(looser).or_insert((0, 0));
                (*loser_entry).1 += 1;

                acc
            });

        let (one_loss, zero_loss): (Vec<_>, Vec<_>) = player_stats
            .iter()
            .filter(|(_, (_, loses))| loses == &0 || loses == &1)
            .partition(|(_, (_, loses))| loses == &1);

        let mut one_loss = one_loss
            .into_iter()
            .map(|(player, _)| *player)
            .collect::<Vec<i32>>();
        let mut zero_loss = zero_loss
            .into_iter()
            .map(|(player, _)| *player)
            .collect::<Vec<i32>>();

        one_loss.sort_unstable();
        zero_loss.sort_unstable();

        vec![zero_loss, one_loss]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![(
            vec![
                vec![1, 3],
                vec![2, 3],
                vec![3, 6],
                vec![5, 6],
                vec![5, 7],
                vec![4, 5],
                vec![4, 8],
                vec![4, 9],
                vec![10, 4],
                vec![10, 9],
            ],
            vec![vec![1, 2, 10], vec![4, 5, 7, 8]],
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::find_winners(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}

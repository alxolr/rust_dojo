use std::fs::read_to_string;

const DRAW: u32 = 3;
const WIN: u32 = 6;
const LOSS: u32 = 0u32;

const X_ROCK: u32 = 1;
const Y_PAPER: u32 = 2;
const Z_SCISSORS: u32 = 3;

fn main() {
    let input_data = read_to_string("./input.data").expect("File not found");

    let calculate_points = |round: &str| {
        let values = round.split(" ").collect::<Vec<_>>();
        let left = values[0];
        let right = values[1];

        match left {
            // ROCK
            "A" => match right {
                "X" => Z_SCISSORS + LOSS,
                "Y" => X_ROCK + DRAW,
                _ => Y_PAPER + WIN,
            },
            // PAPER
            "B" => match right {
                "Y" => Y_PAPER + DRAW,
                "X" => X_ROCK + LOSS,
                _ => Z_SCISSORS + WIN,
            },

            // SCISSORS
            _ => match right {
                "Y" => Z_SCISSORS + DRAW,
                "X" => Y_PAPER + LOSS,
                _ => X_ROCK + WIN,
            },
        }
    };

    let result = input_data.lines().map(calculate_points).sum::<u32>();

    println!("{}", result);
}

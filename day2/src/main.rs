mod game;
mod part1;
mod part2;

use game::Game;
use crate::game::Pull;

const INPUT_PATH: &str = "data/input.txt";

fn main() {
    let input_games = std::fs::read_to_string(INPUT_PATH)
        .expect("failed to read input file at path")
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| parse_line_into_game(s))
        .collect::<Vec<Game>>();

    let part1_score = part1::solve(&input_games);
    let part2_score = part2::solve(&input_games);

    println!("Part 1 score: {part1_score} | Part 2 score: {part2_score}");
}

fn parse_line_into_game(input: String) -> Game {
    let input_parts = input.split(": ");

    let game_id = input_parts.clone()
        .nth(0)
        .expect("did not colon delimiter in input string")
        .split_whitespace()
        .nth(1)
        .expect("did not find game ID in input string")
        .parse::<u32>()
        .expect("failed to parse game ID as integer");

    let pulls = input_parts.clone()
        .nth(1)
        .expect("failed to find pulls in input string")
        .split("; ")
        .map(|s| s.replace('\r', ""))
        .map(|s| Pull::from_str(&s))
        .collect::<Vec<Pull>>();

    Game {
        id: game_id,
        pulls
    }
}
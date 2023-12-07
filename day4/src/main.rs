use std::collections::HashMap;
use crate::game::Game;

mod game;

const INPUT_PATH: &str = "data/input.txt";

fn main() {
    let games = std::fs::read_to_string(INPUT_PATH)
        .expect("failed to read input file at path")
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| Game::from_str(&s))
        .collect::<Vec<Game>>();

    let part1_score: u32 = games
        .iter()
        .map(|game| game.score())
        .sum();

    println!("Part 1 score: {part1_score}");

    let mut card_count: HashMap<u32, u32> = HashMap::new();
    for game in &games {
        card_count.insert(game.id, 1);
    }

    for game in &games {
        let card_num = card_count[&game.id];
        println!("Game {}: {card_num} cards ({} matches)", game.id, game.matches());

        for _ in 0..card_num {
            let matches = game.matches();
            for i in (game.id + 1)..=(game.id + matches as u32) {
                *card_count.get_mut(&i).unwrap() += 1;
            }
        }
    }

    let total_num_cards: u32 = card_count
        .iter()
        .map(|kv| kv.1)
        .sum();

    println!("Part 2 score: {total_num_cards}");
}
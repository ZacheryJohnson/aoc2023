use crate::game::Game;

const TOTAL_RED_CUBES: u32 = 12;
const TOTAL_GREEN_CUBES: u32 = 13;
const TOTAL_BLUE_CUBES: u32 = 14;

pub fn solve(input: &Vec<Game>) -> u32 {
    input
        .iter()
        .map(|game| if is_valid(game) { game.id } else { 0 })
        .sum()
}

fn is_valid(game: &Game) -> bool {
    game
        .pulls
        .iter()
        .all(|pull| pull.total_red <= TOTAL_RED_CUBES && pull.total_green <= TOTAL_GREEN_CUBES && pull.total_blue <= TOTAL_BLUE_CUBES)
}
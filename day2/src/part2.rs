use crate::game::Game;

pub fn solve(input: &Vec<Game>) -> u32 {
    input
        .iter()
        .map(|game| minimum_pull_power(game))
        .sum()
}

pub fn minimum_pull_power(game: &Game) -> u32 {
    let mut max_seen_red = u32::MIN;
    let mut max_seen_green = u32::MIN;
    let mut max_seen_blue = u32::MIN;

    for pull in &game.pulls {
        max_seen_red = u32::max(max_seen_red, pull.total_red);
        max_seen_green = u32::max(max_seen_green, pull.total_green);
        max_seen_blue = u32::max(max_seen_blue, pull.total_blue);
    }

    max_seen_red * max_seen_green * max_seen_blue
}
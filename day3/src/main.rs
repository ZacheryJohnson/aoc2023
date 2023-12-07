mod grid;

use grid::Grid;

const INPUT_PATH: &str = "data/input.txt";

fn main() {
    let input_as_string = std::fs::read_to_string(INPUT_PATH)
        .expect("failed to read input file at path");

    let grid = Grid::from_input(input_as_string);

    let part1_score: u32 = grid.number_tokens
        .iter()
        .filter(|number_token| number_token.is_part(&grid))
        .map(|number_token| number_token.value)
        .sum();

    println!("{part1_score}");
}

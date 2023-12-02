mod part1;
mod part2;

const INPUT_PATH: &str = "data/input.txt";

fn main() {
    let input = std::fs::read_to_string(INPUT_PATH)
        .expect("failed to read input file at path")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let part1_answer = part1::solve(&input);
    let part2_answer = part2::solve(&input);

    println!("Part 1 answer: {part1_answer} | Part 2 answer: {part2_answer}");
}

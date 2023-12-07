use std::collections::HashSet;

#[derive(Debug)]
pub struct Game {
    pub id: u32,

    pub winning_numbers: HashSet<u32>,

    pub owned_numbers: HashSet<u32>,
}

impl Game {
    pub fn from_str(input: &String) -> Game {
        let mut metadata_split = input.split(':');
        let metadata_str = metadata_split.next().unwrap();
        let game_id = metadata_str
            .split_whitespace()
            .nth(1)
            .expect("failed to find game ID")
            .parse::<u32>()
            .expect("failed to parse game ID as integer");

        let mut numbers_split = metadata_split.next().unwrap().split(" | ");
        let winning_str = numbers_split.next().unwrap();
        let winning_numbers = winning_str
            .trim()
            .split_whitespace()
            .map(|number_str| number_str.parse::<u32>().expect("failed to parse winning number as integer"))
            .collect::<HashSet<u32>>();

        let owned_str = numbers_split.next().unwrap();
        let owned_numbers = owned_str
            .trim()
            .split_whitespace()
            .map(|number_str| number_str.parse::<u32>().expect("failed to parse owned number as integer"))
            .collect::<HashSet<u32>>();

        Game {
            id: game_id,
            winning_numbers,
            owned_numbers,
        }
    }

    pub fn matches(&self) -> usize {
        self.winning_numbers
            .intersection(&self.owned_numbers)
            .count()
    }

    pub fn score(&self) -> u32 {
        if self.matches() > 0 {
            2u32.pow((self.matches() - 1) as u32)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_match() {
        let winning = HashSet::from_iter(vec![
            10, 20, 30
        ]);

        let owned = HashSet::from_iter(vec![
            10, 20, 30, 40, 50
        ]);

        let game = Game {
            id: 1,
            winning_numbers: winning,
            owned_numbers: owned
        };

        let expected = 4;
        let actual = game.score();

        assert_eq!(expected, actual);
    }

    #[test]
    fn none_match() {
        let winning = HashSet::from_iter(vec![
            10, 20, 30
        ]);

        let owned = HashSet::from_iter(vec![
            11, 21, 31, 41, 51
        ]);

        let game = Game {
            id: 1,
            winning_numbers: winning,
            owned_numbers: owned
        };

        let expected = 0;
        let actual = game.score();

        assert_eq!(expected, actual);
    }
}
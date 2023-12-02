pub fn solve(input: &Vec<String>) -> u64 {
    input
        .iter()
        // Remove all non-numeric characters from the string
        .map(|input| { let mut copy = input.clone(); copy.retain(|c| c.is_numeric()); copy })
        // Take only the first and last digit
        .map(|numeric_input| [numeric_input.chars().nth(0).unwrap(), numeric_input.chars().nth_back(0).unwrap()] )
        // Parse the characters as a string into a number
        .map(|numeric_digits| numeric_digits.iter().collect::<String>().parse::<u64>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example() {
        let input = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string()
        ];

        let expected = 142;
        let actual = solve(&input);

        assert_eq!(expected, actual);
    }
}
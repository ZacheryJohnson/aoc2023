pub fn solve(input: &Vec<String>) -> u64 {
    input
        .iter()
        // Convert number words into their numeric counterparts
        .map(|input| replace_numeric_words(input.to_owned()))
        // Remove all non-numeric characters from the string
        .map(|input| { let mut stringified = input.to_string(); stringified.retain(|c| c.is_numeric()); stringified })
        // Take only the first and last digit
        .map(|numeric_input| [numeric_input.chars().nth(0).unwrap(), numeric_input.chars().nth_back(0).unwrap()] )
        // Parse the characters as a string into a number
        .map(|numeric_digits| numeric_digits.iter().collect::<String>().parse::<u64>().unwrap())
        .sum()
}

fn try_replace_with_digit(input: &String) -> Option<String> {
    if input.contains("one") {
        Some(input.replacen(input, "1", 1))
    } else if input.contains("two") {
        Some(input.replacen(input, "2", 1))        
    } else if input.contains("three") {
        Some(input.replacen(input, "3", 1))        
    } else if input.contains("four") {
        Some(input.replacen(input, "4", 1))        
    } else if input.contains("five") {
        Some(input.replacen(input, "5", 1))        
    } else if input.contains("six") {
        Some(input.replacen(input, "6", 1))        
    } else if input.contains("seven") {
        Some(input.replacen(input, "7", 1))        
    } else if input.contains("eight") {
        Some(input.replacen(input, "8", 1))        
    } else if input.contains("nine") {
        Some(input.replacen(input, "9", 1))        
    } else {
        None
    }
}

fn replace_numeric_words(mut input: String) -> String {
    let mut forwards_buffer = String::new();
    let mut backwards_buffer = String::new();

    for ch in input.chars() {        
        forwards_buffer.push(ch);

        if ch.is_numeric() {
            input = input.replace(&forwards_buffer, ch.to_string().as_str());
            break;
        }

        if let Some(replacement) = try_replace_with_digit(&forwards_buffer) {
            input = input.replace(&forwards_buffer, &replacement);
            break;
        }
    }

    for ch in input.chars().rev() {
        // Add to front of buffer
        backwards_buffer.insert(0, ch);
        
        if ch.is_numeric() {
            input = input.replace(&backwards_buffer, ch.to_string().as_str());
            break;
        }
        
        if let Some(replacement) = try_replace_with_digit(&backwards_buffer) {
            input = input.replace(&backwards_buffer, &replacement);
            break;
        }
    }

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_input() {
        let expected = "219".to_string();
        let actual = replace_numeric_words("two1nine".to_string());

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_another_input() {
        let expected = "8eight6684".to_string();
        let actual = replace_numeric_words("pbscshhhpeighteight668fourphkdcrjrf".to_string());

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_yet_another_input() {
        let expected = "1two3".to_string();
        let actual = replace_numeric_words("1twothree".to_string());

        assert_eq!(expected, actual);
    }

    #[test]
    fn aoc_example() {
        let input = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];

        let expected = 281;
        let actual = solve(&input);

        assert_eq!(expected, actual);
    }
}
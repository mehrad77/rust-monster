/// Takes a string representing a dice roll in the format of "NdM + X - Y" where
/// N is the number of dice, M is the number of sides on the dice, and X and Y are
/// optional modifiers. Normalizes the input string by converting all uppercase
/// characters to lowercase, removing all whitespace, and ensuring that each
/// dice roll has an explicit "1" before the "d" if no number is provided.
pub fn normalize_dice_roll(dice_roll: &str) -> Result<String, String> {
    // 0. Validate the input string to ensure that it matches the expected format
    if !dice_roll
        .chars()
        .all(|c| c.is_ascii_digit() || c == 'd' || c == 'D' || c == '+' || c == '-' || c == ' ')
    {
        return Err(format!("Invalid input: {}", dice_roll));
    }

    // 1.1. Create an empty string to hold the normalized roll
    let mut normalized_roll = String::new();

    // 1.2. Create an iterator over the characters in the input string
    // and filter out any whitespace characters
    let mut iter = dice_roll.chars().filter(|c| !c.is_whitespace()).peekable();

    // 1.3. add '1' if there is not a digit before the first 'd'
    if dice_roll.starts_with('d') {
        normalized_roll.push('1');
    }

    // Loop over each character in the input string
    while let Some(c) = iter.next() {
        // 2. If `c` is a digit, collect all consecutive digits and add them to the normalized roll
        if c.is_ascii_digit() {
            let mut num = String::new();
            num.push(c);
            // 2.1. Continue reading the rest of the number, digit by digit
            while let Some(d) = iter.peek() {
                if d.is_ascii_digit() {
                    num.push(*d);
                    iter.next();
                } else {
                    break;
                }
            }
            // 2.2. Push it to the normalized string
            normalized_roll.push_str(&num);
        // 3. If `c` is not a digit then
        } else {
            // 3.1. If `c` is a dice declaration (`d/D`)
            if c.to_ascii_lowercase() == 'd' {
                // 3.2. If there is no digit before the 'd' to indicate the count, add '1' as default
                if (normalized_roll
                    .chars()
                    .last()
                    .map_or(true, |prev| !prev.is_ascii_digit()))
                {
                    normalized_roll.push('1');
                }

                // 3.3 append dice declaration
                normalized_roll.push('d');

                // 3.3. add the next character (which should be sides of the dice)
                if let Some(dice) = iter.next() {
                    normalized_roll.push(dice.to_ascii_lowercase());

                    // 3.4. If character after that is a "+" or "-" also add it
                    match iter.peek() {
                        Some('+') | Some('-') => {
                            let sign = iter.next().unwrap();
                            normalized_roll.push(sign);
                        }
                        _ => {}
                    }
                }
            } else {
                // 4. In any other case (usually operators) just append `c`
                normalized_roll.push(c);
            }
        }
    }

    // 5. Remove any trailing '+' or '-' and convert to lowercase
    let normalized_roll = normalized_roll
        .trim_end_matches(|c| c == '+' || c == '-')
        .to_lowercase();

    Ok(normalized_roll)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_dice_roll() -> Result<(), String> {
        // Test case with multiple dice rolls
        assert_eq!(
            normalize_dice_roll("3d6+2+1d6-1-1d8+3")?,
            String::from("3d6+2+1d6-1-1d8+3")
        );
        // Test case with uppercase letters
        assert_eq!(
            normalize_dice_roll("1D20 + 2d4 - 1D6")?,
            String::from("1d20+2d4-1d6")
        );
        // Test case with no modifiers
        assert_eq!(normalize_dice_roll("2d10")?, String::from("2d10"));
        // Test case with whitespace characters
        assert_eq!(
            normalize_dice_roll("2d 6 + 3 - 1")?,
            String::from("2d6+3-1")
        );
        // Test case with only negative modifiers
        assert_eq!(normalize_dice_roll("1d8-2-3")?, String::from("1d8-2-3"));
        // Test case with only positive modifiers
        assert_eq!(normalize_dice_roll("1d8+2+3")?, String::from("1d8+2+3"));
        

        // Test case with invalid input
        let err = normalize_dice_roll("not a valid dice roll").unwrap_err();
        assert_eq!(err, "Invalid input: not a valid dice roll");
        // Test case with no sides on the dice
        // assert_eq!(normalize_dice_roll("1d+4"), String::from("1d4"));
        // Test case with zero-sided dice
        // assert_eq!(normalize_dice_roll("1d0"), String::from("1d1"));
        // Test case with negative dice count
        // assert_eq!(normalize_dice_roll("-2d6+1d4"), String::from("-2d6+1d4"));

        Ok(())
    }
}

// use regex::Regex;

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
                if normalized_roll
                    .chars()
                    .last()
                    .map_or(true, |prev| !prev.is_ascii_digit())
                {
                    normalized_roll.push('1');
                }

                // 3.3 append dice declaration
                normalized_roll.push('d');

                // 3.3. add the next character (which should be sides of the dice)
                if let Some(dice) = iter.next() {
                    if dice.is_ascii_digit() {
                        if dice == '0' {
                            return Err(format!("Invalid dice type: {}", dice_roll));
                        } else {
                            normalized_roll.push(dice);
                        }
                    } else {
                        return Err(format!("Invalid dice type: {}", dice_roll));
                    }

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

fn split_dice_roll(s: &str) -> Vec<String> {
    let mut res = vec![];
    let mut sign = '+';
    let mut num = String::new();

    for c in s.chars() {
        if c.is_digit(10) {
            num.push(c);
        } else if c == 'd' {
            num.push(c);
            continue;
            // res.push(format!("{}{}", sign, num));
            // num = String::new();
        } else if c == '+' || c == '-' {
            if !num.is_empty() {
                res.push(format!("{}{}", sign, num));
                num = String::new();
            }
            sign = c;
        }
    }
    if !num.is_empty() {
        res.push(format!("{}{}", sign, num));
    }
    res
}

#[derive(Debug)]
pub struct DiceEntity {
    pub count: u32,
    pub sides: u32,
    pub sign: char,
    pub is_constant: bool,
    pub value: u32,
}
impl Default for DiceEntity {
    fn default() -> Self {
        Self {
            count: 1,
            sides: 6,
            sign: '+',
            is_constant: false,
            value: 0,
        }
    }
}
impl PartialEq for DiceEntity {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
            && self.sides == other.sides
            && self.sign == other.sign
            && self.is_constant == other.is_constant
            && self.value == other.value
    }
}

fn parse_dice_roll(rolls: Vec<&str>) -> Vec<DiceEntity> {
    let mut res = vec![];

    for roll in rolls {
        let mut count = 0;
        let mut sides = 0;
        let mut sign = '+';
        let mut is_constant = false;
        let mut value = 0;

        let is_dice = roll.contains('d');
        if is_dice {
            let (sign_str, roll) = roll.split_at(1);
            (count, sides) = match roll.split_once('d') {
                Some((count_str, sides_str)) => (
                    count_str.parse::<u32>().unwrap(),
                    sides_str.parse::<u32>().unwrap(),
                ),
                None => panic!("Invalid dice! {}", roll),
            };
            sign = sign_str.chars().next().unwrap_or('+');
            println!(
                "==[dice]==> sign: {}, count: {}, sides: {}",
                sign, count, sides
            );
        } else {
            is_constant = true;

            for c in roll.chars() {
                if c.is_digit(10) {
                    println!("====)))) a digit");
                    // add the digit to the constant value
                    value = value * 10 + c.to_digit(10).unwrap();
                } else if c == 'd' {
                    println!("WRONG!!! WRONG!!! WRONG!!! a 'd'");
                } else if c == '+' || c == '-' {
                    println!("====)))) an operator ({})", c);
                    sign = c;
                }
            }
            println!(
                "==[constant]==> is_constant: {}, sign: {}, value: {}",
                is_constant, sign, value
            );
        }

        let entity = DiceEntity {
            count,
            sides,
            sign,
            is_constant,
            value,
        };
        println!("==[]==> roll: {}, entity: {:?}", roll, entity);
        // Add a new DiceEntity to the result vector with the current values
        res.push(entity);
    }
    res
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
        assert_eq!(normalize_dice_roll("2d10-")?, String::from("2d10"));
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
        let err = normalize_dice_roll("1d+4").unwrap_err();
        assert_eq!(err, "Invalid dice type: 1d+4");
        // Test case with zero-sided dice
        let err = normalize_dice_roll("1d0").unwrap_err();
        assert_eq!(err, "Invalid dice type: 1d0");
        // Test case with invalid character
        let err = normalize_dice_roll("2d6$+3").unwrap_err();
        assert_eq!(err, String::from("Invalid input: 2d6$+3"));

        Ok(())
    }

    #[test]
    fn test_split_dice_roll() {
        assert_eq!(split_dice_roll("1d6"), vec!["+1d6"]);

        assert_eq!(split_dice_roll("2d8-3"), vec!["+2d8", "-3"]);

        assert_eq!(split_dice_roll("2+1d20-1d4"), vec!["+2", "+1d20", "-1d4"]);

        assert_eq!(
            split_dice_roll("1d20+2+3+1d6+1d6-2"),
            vec!["+1d20", "+2", "+3", "+1d6", "+1d6", "-2",]
        );

        assert_eq!(
            split_dice_roll("-1+2+3-4-5+6-7"),
            vec!["-1", "+2", "+3", "-4", "-5", "+6", "-7"]
        );
    }

    #[test]
    fn test_parse_dice_roll() {
        assert_eq!(
            parse_dice_roll(vec!["+1d6"]),
            vec![DiceEntity {
                count: 1,
                sides: 6,
                sign: '+',
                is_constant: false,
                value: 0,
            }]
        );

        assert_eq!(
            parse_dice_roll(vec!["-1d4"]),
            vec![DiceEntity {
                count: 1,
                sides: 4,
                sign: '-',
                is_constant: false,
                value: 0,
            }]
        );

        assert_eq!(
            parse_dice_roll(vec!["+2d8", "-3"]),
            vec![
                DiceEntity {
                    count: 2,
                    sides: 8,
                    sign: '+',
                    is_constant: false,
                    value: 0,
                },
                DiceEntity {
                    count: 0,
                    sides: 0,
                    sign: '-',
                    is_constant: true,
                    value: 3,
                },
            ]
        );

        assert_eq!(
            parse_dice_roll(vec!["+2", "+1d20", "-1d4"]),
            vec![
                DiceEntity {
                    count: 0,
                    sides: 0,
                    sign: '+',
                    is_constant: true,
                    value: 2,
                },
                DiceEntity {
                    count: 1,
                    sides: 20,
                    sign: '+',
                    is_constant: false,
                    value: 0,
                },
                DiceEntity {
                    count: 1,
                    sides: 4,
                    sign: '-',
                    is_constant: false,
                    value: 0,
                },
            ]
        );

        assert_eq!(
            parse_dice_roll(vec!["+1d20", "+2", "+3", "+1d6", "+1d6", "-2"]),
            vec![
                DiceEntity {
                    count: 1,
                    sides: 20,
                    sign: '+',
                    is_constant: false,
                    value: 0,
                },
                DiceEntity {
                    count: 0,
                    sides: 0,
                    sign: '+',
                    is_constant: true,
                    value: 2,
                },
                DiceEntity {
                    count: 0,
                    sides: 0,
                    sign: '+',
                    is_constant: true,
                    value: 3,
                },
                DiceEntity {
                    count: 1,
                    sides: 6,
                    sign: '+',
                    is_constant: false,
                    value: 0,
                },
                DiceEntity {
                    count: 1,
                    sides: 6,
                    sign: '+',
                    is_constant: false,
                    value: 0,
                },
                DiceEntity {
                    count: 0,
                    sides: 0,
                    sign: '-',
                    is_constant: true,
                    value: 2,
                },
            ]
        );
    }
}

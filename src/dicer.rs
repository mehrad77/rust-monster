use rand::Rng;

#[derive(Debug)]
pub struct DiceEntity {
    pub count: u32,
    pub sides: u32,
    pub sign: char,
    pub is_constant: bool,
    pub value: u32,
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

#[derive(Debug)]
pub struct DiceRollResponse {
    /// result of the roll
    pub result: i64,
    /// minimum value to get from this roll
    pub minimum_result: i64,
    /// maximum value to get from this roll
    pub maximum_result: i64,
}
impl PartialEq for DiceRollResponse {
    fn eq(&self, other: &Self) -> bool {
        self.minimum_result == other.minimum_result && self.maximum_result == other.maximum_result
    }
}

/// Takes a string representing a dice roll in the format of "NdM + X - Y" where
/// N is the number of dice, M is the number of sides on the dice, and X and Y are
/// optional modifiers. Normalizes the input string by converting all uppercase
/// characters to lowercase, removing all whitespace, and ensuring that each
/// dice roll has an explicit "1" before the "d" if no number is provided.
fn normalize_dice_roll(dice_roll: &str) -> Result<String, String> {
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

/// Splits a string into a vector of dice roll components.
/// ## Examples
/// ```
/// let s = "2d6+3-1d4";
/// let res = split_dice_roll(s);
/// assert_eq!(res, vec!["+2d6", "+3", "-1d4"]);
/// ```
fn split_dice_roll(input: &str) -> Vec<String> {
    // 1. initialize the variables
    let mut output = vec![];
    let mut sign = '+';
    let mut num_string = String::new();

    // 2. loop the characters of input
    for c in input.chars() {
        if c.is_digit(10) {
            // 2.1. if the `c` is a digit, append it to current string
            num_string.push(c);
        } else if c == 'd' {
            // 2.2. if the `c` is a dice declaration append it to current
            // string and skip the loop for the next character
            num_string.push(c);
            continue;
        } else if c == '+' || c == '-' {
            // 2.3. if the `c` is an operator and is the first character of the
            // current component then it's the sign of the component, but if
            // it's not the first character, it's end of the current component
            if !num_string.is_empty() {
                output.push(format!("{}{}", sign, num_string));
                num_string = String::new();
            }
            sign = c;
        }
    }

    // 3. after the loop, if any character remains, push a new roll component
    if !num_string.is_empty() {
        output.push(format!("{}{}", sign, num_string));
    }

    // 4. return
    output
}

/// Parses a vector of dice roll entries and returns a vector of `DiceEntity` structs.
fn parse_dice_roll(rolls: Vec<&str>) -> Vec<DiceEntity> {
    // 1. initiate the output vector
    let mut output = vec![];

    // 2. loop over the input vector
    for roll in rolls {
        let mut count = 0;
        let mut sides = 0;
        let mut sign = '+';
        let mut is_constant = false;
        let mut value = 0;

        // 3. ...
        let is_dice = roll.contains('d');
        if is_dice {
            // 3.1. Split the roll into sign, count and sides
            let (sign_str, roll) = roll.split_at(1);
            (count, sides) = match roll.split_once('d') {
                Some((count_str, sides_str)) => (
                    count_str.parse::<u32>().unwrap(),
                    sides_str.parse::<u32>().unwrap(),
                ),
                None => panic!("Invalid dice! {}", roll),
            };
            // 3.2. Convert the sign to char
            sign = sign_str.chars().next().unwrap_or('+');
            // println!(
            //     "==[dice]==> sign: {}, count: {}, sides: {}",
            //     sign, count, sides
            // );
        } else {
            // 4. If not a dice, then the roll is a constant value
            is_constant = true;
            for c in roll.chars() {
                if c.is_digit(10) {
                    // 4.1. Add the digit to the constant value
                    value = value * 10 + c.to_digit(10).unwrap();
                } else if c == '+' || c == '-' {
                    // 4.2. Get the sign of the constant value
                    sign = c;
                } else {
                    // 4.3. Panic if there's an unknown character in the constant value
                    panic!("Unknown character: {}", c)
                }
            }
            // println!(
            //     "==[constant]==> is_constant: {}, sign: {}, value: {}",
            //     is_constant, sign, value
            // );
        }

        // 5. Add a new DiceEntity to the result vector with the current values
        let entity = DiceEntity {
            count,
            sides,
            sign,
            is_constant,
            value,
        };
        println!("==[]==> roll: {}, entity: {:?}", roll, entity);
        output.push(entity);
    }

    // 6. return the output
    output
}

fn calculate_dice_entities(entities: Vec<DiceEntity>) -> DiceRollResponse {
    let mut result = 0;
    let mut minimum_result = 0;
    let mut maximum_result = 0;

    for entity in entities {
        if entity.is_constant {
            match entity.sign {
                '+' => {
                    result += entity.value as i64;
                    minimum_result += entity.value as i64;
                    maximum_result += entity.value as i64;
                }
                '-' => {
                    result -= entity.value as i64;
                    minimum_result -= entity.value as i64;
                    maximum_result -= entity.value as i64;
                }
                _ => (),
            }
        } else {
            let mut rng = rand::thread_rng();
            let mut sum = 0;
            for _ in 0..entity.count {
                sum += rng.gen_range(1, entity.sides + 1);
            }
            match entity.sign {
                '+' => {
                    result += sum as i64;
                    minimum_result += entity.count as i64 * 1;
                    maximum_result += entity.count as i64 * entity.sides as i64;
                }
                '-' => {
                    result -= sum as i64;
                    minimum_result -= entity.count as i64 * entity.sides as i64;
                    maximum_result -= entity.count as i64 * 1;
                }
                _ => (),
            }
        }

        println!("=======");
        println!("{:?}", entity);
        println!(
            "r: {}, min: {}, max: {}",
            result, minimum_result, maximum_result,
        );
        println!("=======");
    }

    DiceRollResponse {
        result,
        minimum_result,
        maximum_result,
    }
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

    #[test]
    fn test_calculate_dice_entities() {
        assert_eq!(
            calculate_dice_entities(vec![DiceEntity {
                count: 1,
                sides: 6,
                sign: '+',
                is_constant: false,
                value: 0,
            }]),
            DiceRollResponse {
                // this is RNG and different each time, but doesn't matter in test
                result: 0,
                minimum_result: 1,
                maximum_result: 6,
            }
        );

        assert_eq!(
            calculate_dice_entities(vec![
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
                }
            ]),
            DiceRollResponse {
                // this is RNG and different each time, but doesn't matter in test
                result: 0,
                minimum_result: 2,
                maximum_result: 12,
            }
        );

        assert_eq!(
            calculate_dice_entities(vec![
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
                    sign: '-',
                    is_constant: false,
                    value: 0,
                }
            ]),
            DiceRollResponse {
                // this is RNG and different each time, but doesn't matter in test
                result: 0,
                minimum_result: -5,
                maximum_result: 5,
            }
        );

        assert_eq!(
            calculate_dice_entities(vec![
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
                    value: 5,
                },
                DiceEntity {
                    count: 0,
                    sides: 0,
                    sign: '-',
                    is_constant: true,
                    value: 2,
                },
            ]),
            DiceRollResponse {
                // this is RNG and different each time, but doesn't matter in test
                result: 0,
                minimum_result: 4,
                maximum_result: 23
            }
        );
    }
}

use super::DiceEntity;

/// Parses a vector of dice roll entries and returns a vector of `DiceEntity` structs.
pub fn parse_dice_roll(rolls: Vec<String>) -> Vec<DiceEntity> {
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
        output.push(entity);
    }

    // 6. return the output
    output
}

#[test]
fn test_parse_dice_roll() {
    assert_eq!(
        parse_dice_roll(vec!["+1d6".to_string()]),
        vec![DiceEntity {
            count: 1,
            sides: 6,
            sign: '+',
            is_constant: false,
            value: 0,
        }]
    );

    assert_eq!(
        parse_dice_roll(vec!["-1d4".to_string()]),
        vec![DiceEntity {
            count: 1,
            sides: 4,
            sign: '-',
            is_constant: false,
            value: 0,
        }]
    );

    assert_eq!(
        parse_dice_roll(vec!["+2d8".to_string(), "-3".to_string()]),
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
        parse_dice_roll(vec![
            "+2".to_string(),
            "+1d20".to_string(),
            "-1d4".to_string()
        ]),
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
        parse_dice_roll(vec![
            "+1d20".to_string(),
            "+2".to_string(),
            "+3".to_string(),
            "+1d6".to_string(),
            "+1d6".to_string(),
            "-2".to_string()
        ]),
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

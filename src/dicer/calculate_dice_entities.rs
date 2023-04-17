use rand::Rng;

use super::DiceEntity;

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

pub fn calculate_dice_entities(entities: Vec<DiceEntity>) -> DiceRollResponse {
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
    }

    DiceRollResponse {
        result,
        minimum_result,
        maximum_result,
    }
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

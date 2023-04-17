mod calculate_dice_entities;
mod normalize_dice_roll;
mod parse_dice_roll;
mod split_dice_roll;

/// Rolls a dice using a string input in the format "XdY(+/-)Z", where X is the
/// number of dice to roll, Y is the number of sides on each die, and Z is an
/// optional modifier.
///
/// ## Example
/// ```
/// use rust_monster::dicer::roll_dice;
///
/// let result = roll_dice("2d6+3");
/// assert!(result.total > 0);
/// ```
pub fn roll_dice(input: &str) -> DiceRollResponse {
    let normalized = normalize_dice_roll(input).unwrap();
    let splitted = split_dice_roll(&normalized);
    calculate_dice_entities(parse_dice_roll(splitted))
}

use calculate_dice_entities::*;
use normalize_dice_roll::*;
use parse_dice_roll::*;
use split_dice_roll::*;

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

/// The `roll_error` function takes two arguments:
/// - `dice_roll`: a string representing the input dice roll that caused error.
/// - `msg`: a string slice representing the error message.
/// It returns a formatted string that concatenates the `msg` argument and the
/// `dice_roll` argument, separated by a colon and space.
fn roll_error(dice_roll: &str, msg: &str) -> String {
    format!("{}: {}", msg, dice_roll)
}

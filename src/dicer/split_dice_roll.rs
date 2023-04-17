/// Splits a string into a vector of dice roll components.
/// ## Examples
/// ```
/// let s = "2d6+3-1d4";
/// let res = split_dice_roll(s);
/// assert_eq!(res, vec!["+2d6", "+3", "-1d4"]);
/// ```
pub fn split_dice_roll(input: &str) -> Vec<String> {
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

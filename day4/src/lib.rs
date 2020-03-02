// --- Day 4: Secure Container ---
// You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had written the password on a sticky note, but someone threw it out.

// However, they do remember a few key facts about the password:

// It is a six-digit number.
// The value is within the range given in your puzzle input.
// Two adjacent digits are the same (like 22 in 122345).
// Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679)
// Other than the range rule, the following are true:

// 111111 meets these criteria (double 11, never decreases).
// 223450 does not meet these criteria (decreasing pair of digits 50).
// 123789 does not meet these criteria (no double).
// How many different passwords within the range given in your puzzle input meet these criteria?

// stolen from: https://stackoverflow.com/a/41536521
fn to_digits(number: &i32) -> impl Iterator<Item = u32> {
    return number
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .into_iter();
}

// For a given integer, validate if it fulfills the requirements of a password, based on:
// 1. It is a six-digit number
// 2. Has two adjacent digits
// 3. Digits never decrease
// TODO: Try out a Result-kinda type for shooting values and errors
fn matches(number: &i32) -> bool {
    // 1. It is a six-digit number
    if (111111..=999999).contains(number) == false {
        return false;
    }

    let mut iter = to_digits(number);
    let mut has_adjacent_digits = false;
    let mut digits_never_decrease = false;

    loop {
        let previous = iter.next();
        let item = iter.next();

        // FIXME: Refactor this with `match`
        if let Some(item) = item {
            if let Some(previous) = previous {
                // 3. Digits never decrease
                digits_never_decrease = if previous <= item { true } else { false };

                // 2. Has two adjacent digits
                if item == previous {
                    has_adjacent_digits = true;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    return has_adjacent_digits && digits_never_decrease;
}

// For a given Range, find all possible passwords
pub fn decode(range: std::ops::Range<i32>) -> Vec<i32> {
    return range.filter(|number| matches(number)).collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_finds_possible_passwords() {
        let range: std::ops::Range<i32> = 111111..111114;
        let result: Vec<i32> = decode(range);
        let expected: Vec<i32> = vec![111111, 111112, 111113];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_fails_with_decreasing_pair() {
        let range: std::ops::Range<i32> = 223450..223452;
        let result: Vec<i32> = decode(range);
        let expected: Vec<i32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_fails_with_no_adjacent_pairs() {
        let range: std::ops::Range<i32> = 123789..123798;
        let result: Vec<i32> = decode(range);
        let expected: Vec<i32> = vec![];
        assert_eq!(result, expected);
    }
}

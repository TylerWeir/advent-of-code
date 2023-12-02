/// Returns a number constructed from the first and last digits
///
/// The returned number is constructed such that the first digit in the string populates the tens
/// place and the last digit populates the ones place. Strings containing a single digit will
/// populate both the ones and the tens place with the digit. This function will return None for
/// strings without a digit.
///
/// # Arguments
///
/// * `input` - a string slice to parse the number from
///
/// # Examples
///
/// - `'abc1sd3'` -> `Some(13)`
/// - `'fhcg5s'` -> `Some(55)`
/// - `'powasf'` -> None
///
fn parse_line(input: &str) -> Option<u32> {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;

    for c in input.chars() {
        if c.is_digit(10) {
            match first {
                None => first.replace(c),
                Some(_) => last.replace(c),
            };
        };
    }

    let nums = (first, last);
    match nums {
        (Some(a), Some(b)) => Some(a.to_digit(10).unwrap() * 10 + b.to_digit(10).unwrap()),
        (Some(a), None) => Some(a.to_digit(10).unwrap() * 10 + a.to_digit(10).unwrap()),
        (None, _) => None,
    }
}

/// Returns the sum of the calibration values parsed from each line
///
/// # Arguments
///
/// * `input` - `'\n'` separated string slice to parse calibration values from
fn calc_calibration_sum(input: &str) -> u32 {
    input
        .split('\n')
        .map(parse_line)
        .map(|a| a.unwrap_or(0))
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");
    print!("{}\n", calc_calibration_sum(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_digit() {
        assert_eq!(parse_line("hello"), None);
        assert_eq!(parse_line(""), None);
    }

    #[test]
    fn one_digit() {
        assert_eq!(parse_line("treb7uchet"), Some(77));
    }

    #[test]
    fn two_digits() {
        assert_eq!(parse_line("1abc2"), Some(12));
    }

    #[test]
    fn many_digits() {
        assert_eq!(parse_line("pqr3stu8vwx"), Some(38));
        assert_eq!(parse_line("a1b2c3d4e5f"), Some(15));
    }

    #[test]
    fn full_puzzle_input() {
        let input = include_str!("./input.txt");
        assert_eq!(calc_calibration_sum(input), 54877);
    }
}

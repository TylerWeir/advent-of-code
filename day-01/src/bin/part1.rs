/// Solution for Advent of Code 2023, day 1, part 1.
///
/// Refactor heavily supported by: https://www.youtube.com/watch?v=JOgQMjpGum0

/// Returns a number constructed from the first and last digits
///
/// # Examples
///
/// - `'abc1sd3'` -> `Some(13)`
/// - `'fhcg5s'` -> `Some(55)`
fn parse_line(input: &str) -> u32 {
    let mut nums = input.chars().filter(|c| c.is_digit(10));

    let first = nums.next().expect("Input must contain at least one digit.");
    let last = nums.last();

    match (first, last) {
        (f, Some(l)) => format!("{f}{l}").parse().expect("Failed to parse."),
        (f, None) => format!("{f}{f}").parse().expect("Failed to parse."),
    }
}

/// Returns the sum of the calibration values parsed from each line
///
/// # Arguments
///
/// * `input` - `'\n'` separated string slice to parse calibration values from
fn solve(input: &str) -> u32 {
    input.lines().map(parse_line).sum()
}

fn main() {
    let input = include_str!("./input.txt");
    print!("{}\n", solve(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_parser() {
        assert_eq!(parse_line("treb7uchet"), 77);
        assert_eq!(parse_line("1abc2"), 12);
        assert_eq!(parse_line("pqr3stu8vwx"), 38);
        assert_eq!(parse_line("a1b2c3d4e5f"), 15);
    }

    #[test]
    fn full_puzzle_input() {
        let input = include_str!("./input.txt");
        assert_eq!(solve(input), 54877);
    }
}

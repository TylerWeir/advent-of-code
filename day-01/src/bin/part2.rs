/// Solution for Advent of Code 2023, day 1, part 2.
///
/// Refactor supported by tips from AOC Reddit after initial solution implementation

/// Returns a number constructed from the first and last digits
///
/// # Examples
///
/// - `'abc1sd3'` -> `Some(13)`
/// - `'fhcg5s'` -> `Some(55)`
fn parse_line(input: &str) -> u32 {
    // On release day, the serious snake in the grass was handing overlaps such as "twone" which
    // should become 21. A neat trick is to just embed the desired digit into the middle of the
    // actual word spelling so both get parsed.
    let sanitized = input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "thr3ee")
        .replace("four", "fo4ur")
        .replace("five", "fi5ve")
        .replace("six", "s6x")
        .replace("seven", "se7en")
        .replace("eight", "ei8ght")
        .replace("nine", "ni9ne");

    let mut nums = sanitized.chars().filter(|c| c.is_digit(10));

    let first = nums.next().expect("Input must contain at least one digit.");
    let last = nums.last();

    match (first, last) {
        (f, Some(l)) => format!("{f}{l}").parse().expect("Failed to parse."),
        (f, None) => format!("{f}{f}").parse().expect("Failed to parse."),
    }
}

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
        assert_eq!(parse_line("xtwone3four"), 24);
        assert_eq!(parse_line("4nineeightseven2"), 42);
        assert_eq!(parse_line("zoneight234"), 14);
        assert_eq!(parse_line("7pqrstsixteen"), 76);
        assert_eq!(parse_line("one"), 11);
        // Fear yee
        assert_eq!(parse_line("twone"), 21);
    }

    #[test]
    fn full_puzzle_input() {
        let input = include_str!("./input.txt");
        assert_eq!(solve(input), 54100);
    }
}

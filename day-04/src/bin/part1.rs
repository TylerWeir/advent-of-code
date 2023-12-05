/// Solution for Advent Of Code 2023, Day 4, Part 1.
/// Author: Tyler Weir
///
/// ## Prompt
///
/// Given a input lines: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
///                               [winning nums]        [our nums]
///
/// Per line, one matching num yields a score of 1, subsequent matches double the score.
/// Find the sum of each line's score

fn parse_line(input: &str) -> u32 {
    let nums = input.split_whitespace().skip(2);
    let winning_nums: Vec<u32> = nums
        .clone()
        .take_while(|a| !a.contains("|"))
        .map(|a| a.parse().expect("Failed to parse number"))
        .collect();

    // Fold expression sets score to 1 on the first winning number match, doubles on subsequent
    // matches
    let scorer = |acc, x| {
        if winning_nums.contains(&x) {
            match acc {
                0 => 1,
                _ => acc * 2,
            }
        } else {
            acc
        }
    };

    nums.skip_while(|a| !a.contains("|"))
        .skip(1)
        .map(|a| a.parse().expect("Failed to parse number"))
        .fold(0, scorer)
}

fn solve(input: &str) -> u32 {
    input.lines().map(parse_line).sum()
}

fn main() {
    println!("{}", solve(include_str!("./input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn example_puzzle() {
        assert_eq!(solve(SAMPLE), 13);
    }
}

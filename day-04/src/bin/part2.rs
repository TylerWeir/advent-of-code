/// Solution for Advent Of Code 2023, Day 4, Part 2.
/// Author: Tyler Weir
///
/// ## Prompt
///
/// Given a input lines: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
///                               [winning nums]        [our nums]
///
/// `n` winning numbers causes copies to be made of the next n cards. After evaluating the original
/// cards and all the copies, how many cards do you have?
///
use std::collections::HashMap;

/// Returns the number of winning numbers for a card string
fn parse_line(input: &str) -> u32 {
    let nums = input.split_whitespace().skip(2);
    let winning_nums: Vec<u32> = nums
        .clone()
        .take_while(|a| !a.contains("|"))
        .map(|a| a.parse().expect("Failed to parse number"))
        .collect();

    // Fold expression, counts the number of matches
    let scorer = |acc, x| {
        if winning_nums.contains(&x) {
            acc + 1
        } else {
            acc
        }
    };

    nums.skip_while(|a| !a.contains("|"))
        .skip(1)
        .map(|a| a.parse().expect("Failed to parse number"))
        .fold(0, scorer)
}

fn solver_impl(input: u32, matches: &HashMap<u32, u32>, is_original: bool) -> u32 {
    // TODO(tyler) can this be memoized, I think yes for the copies...
    let num_matches = match matches.get(&input) {
        Some(a) => a,
        None => return 0,
    };
    let mut sum = 1;

    if is_original {
        sum += solver_impl(input + 1, matches, true);
    }

    let lower = input + 1;
    let upper = input + 1 + num_matches;
    for i in lower..upper {
        sum += solver_impl(i, matches, false);
    }

    sum
}

fn solve(input: &str) -> u32 {
    // Find the number of matches for each card
    // TODO can this be lazily evaluated?
    let matches: HashMap<u32, u32> = input
        .lines()
        .map(parse_line)
        .enumerate()
        .map(|(line, val)| (line as u32 + 1, val))
        .collect();

    solver_impl(1, &matches, true)
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
        assert_eq!(solve(SAMPLE), 30);
    }

    #[test]
    fn full_puzzle() {
        assert_eq!(solve(include_str!("./input.txt")), 5095824);
    }
}

/// Returns the total distance traveled by the boat
fn calc_distance(held: u32, total: u32) -> u32 {
    (total - held) * held
}

/// Returns the times the boat could be held that would beat the record distance
///
/// TODO: Because the winning times are always continuous, this could just return a range
/// Intercepts can be found with (t +- (t^2 - 4c)^1/2 /2) where t is total race time, c is record
fn calc_winning_modes(time: u32, distance: u32) -> Vec<u32> {
    let possible_times = 0..time;

    possible_times
        .filter(|&a| calc_distance(a, time) > distance)
        .collect()
}

/// Solves the puzzle for the provided input
///
/// TODO: Clean up the parsing
fn solve(input: &str) -> u32 {
    let mut input_itr = input.split('\n');

    let times = input_itr
        .nth(0)
        .expect("invalid input")
        .split_whitespace()
        .skip(1)
        .map(|a| u32::from_str_radix(a, 10).expect("failed to parse integer"));
    let distances = input_itr
        .nth(0)
        .expect("invalid input")
        .split_whitespace()
        .skip(1)
        .map(|a| u32::from_str_radix(a, 10).expect("failed to parse integer"));

    times
        .zip(distances)
        .map(|(a, b)| calc_winning_modes(a, b).len() as u32)
        .fold(1, |acc, x| acc * x)
}

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", solve(input));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn actual_puzzle() {
        let input = include_str!("./input.txt");
        assert_eq!(solve(input), 128700);
    }
}

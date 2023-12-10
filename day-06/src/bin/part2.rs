/// Returns the total distance traveled by the boat
fn calc_distance(held: u64, total: u64) -> u64 {
    (total - held) * held
}

/// Returns the times the boat could be held that would beat the record distance
///
/// TODO: Because the winning times are always continuous, this could just return a range
/// Intercepts can be found with (t +- (t^2 - 4c)^1/2 /2) where t is total race time, c is record
fn calc_winning_modes(time: u64, distance: u64) -> Vec<u64> {
    let possible_times = 0..time;

    possible_times
        .filter(|&a| calc_distance(a, time) > distance)
        .collect()
}

/// Solves the puzzle for the provided input
///
/// TODO: Clean up the parsing
fn solve(input: &str) -> u64 {
    let mut input_itr = input.split('\n');

    let time: u64 = input_itr
        .nth(0)
        .expect("invalid input")
        .strip_prefix("Time:")
        .expect("Failed to parse")
        .replace(" ", "")
        .parse()
        .expect("Failed");

    let distance: u64 = input_itr
        .nth(0)
        .expect("invalid input")
        .strip_prefix("Distance:")
        .expect("Failed to parse")
        .replace(" ", "")
        .parse()
        .expect("Failed");

    calc_winning_modes(time, distance).len() as u64
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
        assert_eq!(solve(input), 39594072);
    }
}

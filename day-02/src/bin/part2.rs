use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Game {
    gid: u32,
    red: u32,
    blue: u32,
    green: u32,
}

impl Game {
    fn calc_power(self) -> u32 {
        self.red * self.blue * self.green
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Parse the id
        let num_end = input.find(':').unwrap();
        let (first, last) = input.split_at(num_end);

        let id = u32::from_str(first.strip_prefix("Game ").unwrap())
            .ok()
            .unwrap();

        let mut max_colors: HashMap<String, u32> = HashMap::new();

        let games = last.strip_prefix(":").unwrap().split(";");
        for game in games {
            let scrubbed = game.trim().replace(',', "");
            let items = scrubbed.split_whitespace().map(String::from);

            let numbers = items
                .clone()
                .step_by(2)
                .map(|a| u32::from_str(a.as_str()).unwrap());
            let colors = items.clone().skip(1).step_by(2);

            let map_entry = colors.zip(numbers);

            for entry in map_entry {
                if max_colors.contains_key(&entry.0) {
                    // only add if bigger
                    if entry.1 > *max_colors.get(&entry.0).unwrap() {
                        max_colors.insert(entry.0, entry.1);
                    }
                } else {
                    max_colors.insert(entry.0, entry.1);
                }
            }
        }
        let alt: u32 = 0;

        return Ok(Game {
            gid: id,
            blue: *max_colors.get("blue").unwrap_or(&alt),
            red: *max_colors.get("red").unwrap_or(&alt),
            green: *max_colors.get("green").unwrap_or(&alt),
        });
    }
}

fn calc_solution(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|a| !a.is_empty())
        .map(Game::from_str)
        .map(Result::ok)
        .map(Option::unwrap)
        .map(Game::calc_power)
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", calc_solution(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_from_str() {
        let actual = Game::from_str("Game 11: 12 blue, 4 red; 10 red, 13 green, 6 blue; 2 green");
        let expected = Game {
            gid: 11,
            red: 10,
            blue: 12,
            green: 13,
        };
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn other_game_from_str() {
        let actual = Game::from_str("Game 79: 4 red, 3 green; 3 blue, 10 green, 4 red; 1 red, 12 green, 7 blue; 5 blue, 3 green, 6 red; 10 green, 1 blue, 5 red; 5 green, 5 red");
        let expected = Game {
            gid: 79,
            red: 6,
            blue: 7,
            green: 12,
        };
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn sample_solution() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(calc_solution(input), 2286);
    }
}

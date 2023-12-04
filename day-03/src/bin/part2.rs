#[derive(Debug)]
struct Point {
    pub row: usize,
    pub col: usize,
}

/// Returns the value of the number intercepted by the given index. `None` if out of bounds or non
/// digit.
fn num_from_str(idx: usize, input: &str) -> Option<u32> {
    // this should catch bounds errors
    let mut base: String = match input.chars().nth(idx) {
        Some(a) => a.to_string(),
        None => return None,
    };

    // Catch number errors
    if !input.chars().nth(idx).unwrap().is_digit(10) {
        return None;
    }

    // look leftwards
    let mut index;
    if idx > 0 {
        index = idx - 1;
        loop {
            match input.chars().nth(index) {
                Some(a) => {
                    if a.is_digit(10) {
                        base.insert(0, a);
                        if index > 0 {
                            index -= 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
    }

    // look rightwards
    index = idx + 1;
    loop {
        match input.chars().nth(index) {
            Some(a) => {
                if a.is_digit(10) {
                    index += 1;
                    base.push(a);
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    Some(u32::from_str_radix(&base, 10).unwrap())
}

/// Returns the value of the number intercepted by the given index. `None` if out of bounds or non
/// digit.
fn get_num_at(p: Point, schematic: &Vec<&str>) -> Option<u32> {
    // Guard on row
    if p.row >= schematic.len() {
        return None;
    }

    num_from_str(p.col, schematic[p.row])
}

// Returns the locations surrounding a span at a given position.
fn get_surrounding_points(pos: Point, width: usize) -> Vec<Point> {
    let shifted_pos = Point {
        row: pos.row + 1,
        col: pos.col + 1,
    };

    let mut points: Vec<Point> = vec![];

    // left edge
    points.push(Point {
        row: shifted_pos.row,
        col: shifted_pos.col - 1,
    });

    // right edge
    points.push(Point {
        row: shifted_pos.row,
        col: shifted_pos.col + width,
    });

    // Add two for the corners
    for i in 0..width + 2 {
        // top
        points.push(Point {
            row: shifted_pos.row - 1,
            col: shifted_pos.col - 1 + i,
        });
        // bottom
        points.push(Point {
            row: shifted_pos.row + 1,
            col: shifted_pos.col - 1 + i,
        });
    }

    // remove invalid points and shift back
    points
        .iter()
        .filter(|a| a.row != 0 && a.col != 0)
        .map(|a| Point {
            row: a.row - 1,
            col: a.col - 1,
        })
        .collect()
}

/// Returns the indices of all `*` characters
fn find_gears(input: &str) -> Vec<usize> {
    input
        .match_indices(|a: char| a == '*')
        .map(|(a, _)| a)
        .collect()
}

fn solve(input: &str) -> u32 {
    let schematic: Vec<&str> = input.split("\n").filter(|a| !a.is_empty()).collect();

    let mut running_sum = 0;

    // Find the Gear locations
    let mut gear_locations: Vec<Point> = vec![];
    for row in 0..schematic.len() {
        let gear_cols = find_gears(schematic[row]);

        for col in gear_cols {
            gear_locations.push(Point { row: row, col: col });
        }
    }

    for gear in gear_locations {
        let mut nums: Vec<u32> = vec![];

        // Find the neighboring numbers
        let neigbors = get_surrounding_points(gear, 1);
        for neigbor in neigbors {
            match get_num_at(neigbor, &schematic) {
                Some(a) => nums.push(a),
                None => (),
            }
        }

        // Clear duplicates since a multiple digits of one number can be adjacent to a gear
        nums.sort();
        nums.dedup();

        if nums.len() == 2 {
            running_sum += nums[0] * nums[1];
        }
    }

    running_sum
}

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", solve(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn num_from_index() {
        assert_eq!(num_from_str(0, "100...."), Some(100));
        assert_eq!(num_from_str(2, "467...."), Some(467));
        assert_eq!(num_from_str(3, "..324..."), Some(324));

        assert_eq!(num_from_str(100, "..."), None);
        assert_eq!(num_from_str(3, "....."), None);
    }

    #[test]
    fn example_puzzle() {
        assert_eq!(solve(PUZZLE_EXAMPLE), 467835);
    }
}

#[derive(Debug)]
struct Point {
    pub row: usize,
    pub col: usize,
}

fn get_num_length(input: impl Iterator<Item = char>) -> usize {
    let number = input.take_while(|a| a.is_digit(10));
    number.count()
}

fn get_num_value(input: impl Iterator<Item = char>) -> u32 {
    let number = input.take_while(|a| a.is_digit(10));
    let num_str: String = number.collect();
    u32::from_str_radix(&num_str, 10).unwrap()
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

fn is_location_symbol(location: Point, schematic: &Vec<&str>) -> bool {
    let max_col = schematic[0].len() - 1;
    let max_row = schematic.len() - 1;

    // Guard on being in range
    if location.row > max_row || location.col > max_col {
        return false;
    } else {
        return is_symbol(
            schematic
                .get(location.row)
                .unwrap()
                .chars()
                .nth(location.col)
                .unwrap(),
        );
    }
}

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

fn is_valid_num(location: Point, schematic: &Vec<&str>) -> bool {
    // Make sure we're starting on a number
    assert!(schematic
        .get(location.row)
        .unwrap()
        .chars()
        .nth(location.col)
        .unwrap()
        .is_digit(10));

    // Commence the search for the symbol
    let width = get_num_length(
        schematic
            .get(location.row)
            .unwrap()
            .chars()
            .skip(location.col),
    );

    for neighbor in get_surrounding_points(location, width) {
        if is_location_symbol(neighbor, schematic) {
            return true;
        }
    }

    false
}

fn find_num_starts(input: &str) -> Vec<usize> {
    let indices: Vec<usize> = input
        .match_indices(|a: char| a.is_digit(10))
        .map(|(a, _)| a)
        .collect();

    let mut starts: Vec<usize> = vec![];
    for idx in &indices {
        if *idx == 0 || !indices.contains(&(idx - 1)) {
            starts.push(*idx);
        }
    }
    starts
}

fn solve(input: &str) -> u32 {
    let schematic: Vec<&str> = input.split("\n").filter(|a| !a.is_empty()).collect();

    let mut running_sum = 0;

    for row in 0..schematic.len() {
        let num_starts = find_num_starts(schematic[row]);

        for col in num_starts {
            let location = Point { row: row, col: col };
            if is_valid_num(location, &schematic) {
                running_sum += get_num_value(schematic[row].chars().skip(col));
            }
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
    const TEST_STR: &str = "123..67.9";
    const TEST_SCHEMA: &str = "..34....32
..42*.*...
......21..";

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
    fn num_length() {
        assert_eq!(get_num_length(TEST_STR.chars()), 3);
        assert_eq!(get_num_length(TEST_STR.chars().skip(5)), 2);
        assert_eq!(get_num_length(TEST_STR.chars().skip(8)), 1);
    }

    #[test]
    fn num_value() {
        assert_eq!(get_num_value(TEST_STR.chars()), 123);
        assert_eq!(get_num_value(TEST_STR.chars().skip(5)), 67);
        assert_eq!(get_num_value(TEST_STR.chars().skip(8)), 9);
    }

    #[test]
    fn symbol_checks() {
        assert!(is_symbol('*'));
        assert!(!is_symbol('.'));
        assert!(!is_symbol('4'));
    }

    #[test]
    fn valid_nums() {
        let schematic: Vec<&str> = TEST_SCHEMA.split('\n').collect();

        // positives
        assert!(is_valid_num(Point { row: 1, col: 2 }, &schematic));
        assert!(is_valid_num(Point { row: 2, col: 6 }, &schematic));
        assert!(is_valid_num(Point { row: 0, col: 2 }, &schematic));

        // negatives
        assert!(!is_valid_num(Point { row: 0, col: 8 }, &schematic));
    }

    #[test]
    fn sums() {
        assert_eq!(solve(PUZZLE_EXAMPLE), 4361);
    }

    #[test]
    fn find_num_starts_tests() {
        assert_eq!(find_num_starts("1...."), vec![0]);
        assert_eq!(find_num_starts(".012.."), vec![1]);
        assert_eq!(find_num_starts(".02..3..423324"), vec![1, 5, 8]);
    }

    #[test]
    fn actual_puzzle() {
        let input = include_str!("./input.txt");
        assert_eq!(solve(input), 527144)
    }
}

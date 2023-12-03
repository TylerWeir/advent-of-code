use std::collections::HashMap;

fn get_num(c: &char) -> Option<u32> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    }
}

fn as_char(s: &str) -> Option<&str> {
    match s {
        "one" => Some("1"),
        "two" => Some("2"),
        "three" => Some("3"),
        "four" => Some("4"),
        "five" => Some("5"),
        "six" => Some("6"),
        "seven" => Some("7"),
        "eight" => Some("8"),
        "nine" => Some("9"),
        _ => None,
    }
}

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn filter_words(s: &String) -> String {
    let mut input = s.clone();

    let mut words = HashMap::new();

    loop {
        for w in WORDS {
            let location = input.find(w);
            match location {
                Some(a) => words.insert(w, a),
                None => None,
            };
        }

        if words.is_empty() {
            println!("{s} ---> {input}");
            return input;
        }

        let closest_word = words
            .clone()
            .into_iter()
            .reduce(|acc, e| if acc.1 < e.1 { acc } else { e })
            .get_or_insert(("fuuuuuuck", 1))
            .0;

        input = input.replace(closest_word, as_char(closest_word).get_or_insert("----"));

        words.clear();
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let value: u32 = input.split('\n').map(parse_line).sum();
    print!("{value}\n");
}

fn is_num(c: &char) -> bool {
    get_num(c).is_some()
}

fn find_first(inp: &str) -> Option<u32> {
    // convert words to chars
    let input_str = inp.to_string();
    let input_s = filter_words(&input_str);
    let input = input_s.as_str();

    for w in input.chars() {
        if get_num(&w).is_some() {
            return get_num(&w);
        }
    }
    None
}

fn find_last(inp: &str) -> u32 {
    let mut target_str = String::new();

    for c in inp.chars().rev() {
        target_str.insert(0, c);

        let maybe_first = find_first(target_str.as_str());

        if maybe_first.is_some() {
            return maybe_first.unwrap();
        }
    }
    panic!("emtpy string in find last!")
}

fn parse_line(inp: &str) -> u32 {
    if inp == "" {
        return 0;
    }

    let first = find_first(inp).unwrap();
    let last = find_last(inp);

    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_digit() {
        assert_eq!(parse_line("treb7uchet"), 77);
    }

    #[test]
    fn two_digits() {
        assert_eq!(parse_line("1abc2"), 12);
    }

    #[test]
    fn many_digits() {
        assert_eq!(parse_line("pqr3stu8vwx"), 38);
        assert_eq!(parse_line("a1b2c3d4e5f"), 15);
    }

    #[test]
    fn two_words() {
        assert_eq!(parse_line("two1nine"), 29);
    }

    #[test]
    fn only_words() {
        assert_eq!(parse_line("eightwothree"), 83);
    }

    #[test]
    fn gibberish_and_words() {
        assert_eq!(parse_line("abcone2threexyz"), 13)
    }

    #[test]
    fn the_nail() {
        assert_eq!(parse_line("18oneight"), 18)
    }

    #[test]
    fn others() {
        assert_eq!(parse_line("xtwone3four"), 24);
        assert_eq!(parse_line("4nineeightseven2"), 42);
        assert_eq!(parse_line("zoneight234"), 14);
        assert_eq!(parse_line("7pqrstsixteen"), 76);
        assert_eq!(parse_line(""), 0);
        assert_eq!(parse_line("one"), 11);
    }
}

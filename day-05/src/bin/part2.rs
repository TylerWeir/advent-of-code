// Maps are in order... lets just make a vector of these 'map' types, then fold input number over
// the vector of maps, which should result in the output type
//
// really only need to store the offset value, that the source ranges map to

// How to map ranges to values?
use rangemap::RangeMap;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
struct Map {
    mappings: RangeMap<i64, i64>,
}

impl Map {
    fn get_destination(&self, input: i64) -> i64 {
        match self.mappings.get(&input) {
            Some(a) => input + a,
            None => input,
        }
    }

    fn new() -> Map {
        Map {
            mappings: RangeMap::new(),
        }
    }
}

#[derive(Debug)]
struct MapParseError {}

impl FromStr for Map {
    type Err = MapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split('\n').skip(1);

        let mut map = Map::new();

        for l in data {
            if l.len() == 0 {
                continue;
            }
            let mut nums = l.split(" ");
            let dest: i64 = nums.nth(0).expect("Error").parse().expect("Parse Error");
            let source: i64 = nums.nth(0).expect("Error").parse().expect("Parse Error");
            let range: i64 = nums.nth(0).expect("Error").parse().expect("Parse Error");

            let upper_bound = source + range;
            let offset = dest - source;

            map.mappings.insert(source..upper_bound, offset);
        }

        Ok(map)
    }
}

fn solve(input: &str) -> i64 {
    let mut iter = input.split("\n\n");

    let nums = iter
        .nth(0)
        .expect("Failed to get nums")
        .split(' ')
        .skip(1)
        .map(|a| i64::from_str(a).expect("failed to parse"));

    let nums_e = nums.clone().step_by(2);
    let nums_o = nums.clone().skip(1).step_by(2);
    let nums_z = nums_e.zip(nums_o);

    let mut actual_nums: Vec<i64> = vec![];

    for (s, e) in nums_z {
        let ceiling = s + e;
        for n in s..ceiling {
            actual_nums.push(n);
        }
    }

    let almanac: Vec<Map> = iter
        .map(|a| Map::from_str(a).expect("Failed to parse"))
        .collect();

    let location_finder = |acc, x: &Map| x.get_destination(acc);

    actual_nums
        .into_iter()
        .map(|a| almanac.iter().fold(a, location_finder))
        .min()
        .expect("num")
}

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", solve(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapr_from_str() {
        let input = "seed-to-soil map:
50 98 2
52 50 48";

        let mut mappings = RangeMap::new();
        mappings.insert(98..100, -48);
        mappings.insert(50..98, 2);
        let expected = Map { mappings: mappings };
        assert_eq!(Map::from_str(input).expect("Failed to parse"), expected);
    }

    #[test]
    fn get_destination() {
        let mut mappings = RangeMap::new();
        mappings.insert(98..100, -48);
        mappings.insert(50..98, 2);
        let my_map = Map { mappings: mappings };

        assert_eq!(my_map.get_destination(98), 50);
    }

    #[test]
    fn sample_puzzle() {
        let sample_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(solve(sample_input), 46);
    }
}

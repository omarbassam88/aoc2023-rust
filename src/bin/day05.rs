use std::fs;

fn main() {
    let day = 5;
    println!("Solution to Day {day}");
    let input = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

type RangesMap = Vec<Vec<u64>>;

fn parse_range(line: &str) -> Vec<u64> {
    line.split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

fn parse_map(s: &str) -> RangesMap {
    let mut s_iter = s.split(" map:\n");
    let data = s_iter.nth(1).unwrap().lines().map(parse_range).collect();
    data
}

fn part1(input: &String) -> u64 {
    let mut input_iter = input.split("\n\n");
    let seeds: Vec<u64> = input_iter
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    let mut maps: Vec<RangesMap> = vec![];
    while let Some(entry) = input_iter.next() {
        let data = parse_map(entry);
        maps.push(data);
    }

    let mut min_loc = std::u64::MAX;
    for seed in seeds {
        let mut current = seed;
        for ranges in &maps {
            for range in ranges {
                if range[1] <= current && current < range[1] + range[2] {
                    let val = current - range[1] + range[0];
                    current = val;
                    break;
                }
            }
        }
        if current < min_loc {
            min_loc = current;
        }
    }

    min_loc
}

fn part2(input: &String) -> u64 {
    let mut input_iter = input.split("\n\n");
    let seeds_input: Vec<u64> = input_iter
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    let seeds_ranges: Vec<_> = seeds_input.chunks(2).collect();
    let mut maps: Vec<Vec<Vec<u64>>> = vec![];
    while let Some(entry) = input_iter.next() {
        let data = parse_map(entry);
        maps.push(data);
    }

    let mut min_loc = std::u64::MAX;
    for seed_range in &seeds_ranges {
        for seed in seed_range[0]..seed_range[0] + seed_range[1] {
            let mut current = seed;
            for ranges in &maps {
                for range in ranges {
                    if range[1] <= current && current < range[1] + range[2] {
                        let val = current - range[1] + range[0];
                        current = val;
                        break;
                    }
                }
            }
            if current < min_loc {
                min_loc = current;
            }
        }
    }

    min_loc
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"seeds: 79 14 55 13

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

    #[test]
    fn part1_test() {
        let result = part1(&INPUT.to_string());
        assert_eq!(result, 35);
    }

    #[test]
    fn part2_test() {
        let result = part2(&INPUT.to_string());
        assert_eq!(result, 46);
    }
}

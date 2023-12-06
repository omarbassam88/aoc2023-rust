use std::collections::HashMap;
use std::fs;

fn main() {
    let day = 5;
    println!("Solution to Day {day}");
    let input = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn parse_range(line: &str) -> Vec<u64> {
    line.split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

fn parse_map(s: &str) -> (&str, Vec<Vec<u64>>) {
    let mut s_iter = s.split(" map:\n");
    let title = s_iter.next().unwrap();
    let data = s_iter.next().unwrap().lines().map(parse_range).collect();
    (title, data)
}

fn part1(input: &String) -> u64 {
    let mut input_iter = input.split("\n\n");
    let mut seeds: Vec<u64> = input_iter
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    let mut maps: Vec<Vec<Vec<u64>>> = vec![];
    while let Some(entry) = input_iter.next() {
        let (_, data) = parse_map(entry);
        maps.push(data);
    }

    for i in 0..seeds.len() {
        for map in &maps {
            for range in map {
                if range[1] < seeds[i] && seeds[i] < range[1] + range[2] {
                    let val = seeds[i] - range[1] + range[0];
                    seeds[i] = val;
                    break;
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

fn part2(input: &String) -> u64 {
    let mut input_iter = input.split("\n\n");
    let seeds_ranges: Vec<u64> = input_iter
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    let mut maps: Vec<Vec<Vec<u64>>> = vec![];
    while let Some(entry) = input_iter.next() {
        let (_, data) = parse_map(entry);
        maps.push(data);
    }

    let mut seeds = vec![];
    for i in (0..seeds_ranges.len()).step_by(2) {
        let start = seeds_ranges[i];
        let end = start + seeds_ranges[i + 1];
        for j in start..end {
            seeds.push(j);
        }
    }
    let mappings: Vec<HashMap<u64, u64>> = maps
        .iter()
        .map(|ranges| {
            let mut m = HashMap::new();
            for range in ranges {
                let dest_start = range[0];
                let source_start = range[1];
                let offset = range[2];
                for i in 0..offset {
                    m.insert(source_start + i, dest_start + i);
                }
            }
            m
        })
        .collect();
    for i in 0..seeds.len() {
        for map in &mappings {
            seeds[i] = *map.get(&seeds[i]).unwrap_or(&seeds[i]);
        }
    }

    *seeds.iter().min().unwrap()
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

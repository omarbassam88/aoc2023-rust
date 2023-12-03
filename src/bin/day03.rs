use regex::Regex;
use std::fs;

fn main() {
    let day: u32 = 3;
    println!("Solution to Day {day}");
    let input: String = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn part1(input: &String) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;
    for i in 0..lines.len() {
        let line = lines[i];
        // A part is one or more digit(s)
        let rx = Regex::new(r"\d+").unwrap();
        // A symbol is not a number and not a "."
        let symbols_regex = Regex::new(r"[^0-9\.]").unwrap();
        for m in rx.find_iter(line) {
            let num: u32 = m.as_str().parse().unwrap();
            let from = if m.start() > 0 { m.start() - 1 } else { 0 };
            let to = if m.end() < line.len() {
                m.end()
            } else {
                line.len() - 1
            };
            // Look Left and Right
            let s = &line[from..=to];
            if symbols_regex.is_match(s) {
                sum += num;
                continue;
            };
            // Look Up
            if i > 0 {
                let s = &lines[i - 1][from..=to];
                if symbols_regex.is_match(s) {
                    sum += num;
                    continue;
                };
            }
            // Look Down
            if i < (lines.len() - 1) {
                let s = &lines[i + 1][from..=to];
                if symbols_regex.is_match(s) {
                    sum += num;
                    continue;
                };
            }
        }
    }
    sum
}

fn part2(input: &String) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;
    for i in 0..lines.len() {
        let line = lines[i];
        let gears: Vec<usize> = line.match_indices("*").map(|(i, _)| i).collect();
        let rx = Regex::new(r"\d+").unwrap();

        // helper function to check if part is adjacent to the gear
        let is_adjacent = |gear, start, end| {
            start == gear - 1
                || start == gear
                || start == gear + 1
                || end == gear
                || end == gear + 1
                || end == gear + 2
        };

        for gear in gears {
            let mut parts: Vec<u32> = vec![];
            // Look Left and Right
            for m in rx.find_iter(line) {
                let num: u32 = m.as_str().parse().unwrap();
                if m.start() == gear + 1 || m.end() == gear {
                    parts.push(num);
                }
            }
            // Look Up
            if i > 0 {
                for m in rx.find_iter(lines[i - 1]) {
                    let num: u32 = m.as_str().parse().unwrap();
                    if is_adjacent(gear, m.start(), m.end()) {
                        parts.push(num);
                    }
                }
            }
            // Look Down
            if i < lines.len() {
                for m in rx.find_iter(lines[i + 1]) {
                    let num: u32 = m.as_str().parse().unwrap();
                    if is_adjacent(gear, m.start(), m.end()) {
                        parts.push(num);
                    }
                }
            }
            // A gear must only have two parts
            if parts.len() == 2 {
                sum += parts[0] * parts[1];
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"467..114..
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
    fn part1_test() {
        let result = part1(&INPUT.to_string());
        assert_eq!(result, 4361);
    }

    #[test]
    fn part2_test() {
        let result = part2(&INPUT.to_string());
        assert_eq!(result, 467835);
    }
}

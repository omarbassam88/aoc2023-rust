use regex::Regex;
use std::fs;

fn main() {
    let day = 6;
    println!("Solution to Day {day}");
    let input = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn part1(input: &String) -> u32 {
    let rx = Regex::new(r"\d+").unwrap();
    let mut lines = input.lines();
    let durations = rx
        .find_iter(lines.next().unwrap())
        .map(|m| m.as_str().parse::<u32>().unwrap());
    let records = rx
        .find_iter(lines.next().unwrap())
        .map(|m| m.as_str().parse::<u32>().unwrap());

    let mut wins: u32 = 1;
    let mut ways;
    for (duration, record) in durations.zip(records) {
	ways = 0;
        for hold in 1..duration {
            let distance = (duration - hold) * hold;
            if distance > record {
                ways += 1;
            };
        }
        wins *= ways;
    }

    wins
}

fn part2(input: &String) -> u64 {
    let mut lines = input.lines();
    let duration: u64 = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .replace(" ", "")
        .parse()
        .unwrap();
    let record: u64 = lines
        .next()
        .unwrap()
        .replace("Distance:", "")
        .replace(" ", "")
        .parse()
        .unwrap();
    let mut ways = 0;
    for hold in 1..duration {
        let distance = (duration - hold) * hold;
        if distance > record {
            ways += 1;
        };
    }

    ways
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_test() {
        let result = part1(&INPUT.to_string());
        assert_eq!(result, 288);
    }

    #[test]
    fn part2_test() {
        let result = part2(&INPUT.to_string());
        assert_eq!(result, 71503);
    }
}

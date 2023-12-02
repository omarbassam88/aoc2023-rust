use std::{cmp::max, fs, str::FromStr};

fn main() {
    let day: u32 = 2;
    println!("Solution to Day {day}");
    let input: String = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

struct Game {
    id: u32,
    subsets: Vec<SubSet>,
}

type SubSet = Vec<Round>;
type Round = (Color, u32);

enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

#[derive(Debug, PartialEq, Eq)]
struct ParseColorError;

impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line_trim: String = line.replace("Game ", "");
        let mut id_subsets = line_trim.split(": ");
        let id: u32 = id_subsets.next().unwrap().parse().unwrap();
        let subsets_str = id_subsets.next().unwrap();
        let subsets: Vec<SubSet> = subsets_str.split("; ").map(parse_subset).collect();

        Ok(Game { id, subsets })
    }
}

fn parse_subset(s: &str) -> SubSet {
    s.split(", ")
        .map(|p| -> Round {
            let mut it = p.split(" ");
            let count: u32 = it.next().unwrap().parse().unwrap();
            let color: Color = it.next().unwrap().parse().unwrap();
            (color, count)
        })
        .collect()
}

impl FromStr for Color {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(ParseColorError),
        }
    }
}

fn part1(input: &String) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut possible: bool = true;
        let game: Game = line.parse().unwrap();

        for subset in &game.subsets {
            for (color, count) in subset {
                match (color, count) {
                    (Color::Red, (12..)) => possible = false,
                    (Color::Green, (13..)) => possible = false,
                    (Color::Blue, (14..)) => possible = false,
                    (_, _) => {}
                }
            }
        }

        if possible {
            sum += game.id;
        };
    }
    sum
}

fn part2(input: &String) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let game: Game = line.parse().unwrap();
        let mut min_red: u32 = 1;
        let mut min_green: u32 = 1;
        let mut min_blue: u32 = 1;

        for subset in &game.subsets {
            for (color, count) in subset {
                match color {
                    Color::Red => min_red = max::<u32>(*count, min_red),
                    Color::Green => min_green = max::<u32>(*count, min_green),
                    Color::Blue => min_blue = max::<u32>(*count, min_blue),
                }
            }
        }
        sum += min_red * min_green * min_blue;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_test() {
        let result = part1(&INPUT.to_string());
        assert_eq!(result, 8);
    }

    #[test]
    fn part2_test() {
        let result = part2(&INPUT.to_string());
        assert_eq!(result, 2286);
    }
}

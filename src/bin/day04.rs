use std::collections::HashSet;
use std::fs;

fn main() {
    let day: u32 = 4;
    println!("Solution to Day {day}");
    let input: String = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn parse_numbers(s: &str) -> HashSet<u32> {
    s.split(" ").filter_map(|s| s.parse::<u32>().ok()).collect()
}

fn parse_card(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let mut line_iter = line.split(": ");
    let mut card_data = line_iter.nth(1).unwrap().split(" | ");
    let winning_nums: HashSet<u32> = parse_numbers(card_data.next().unwrap());
    let my_nums: HashSet<u32> = parse_numbers(card_data.next().unwrap());

    (winning_nums, my_nums)
}

fn part1(input: &String) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (winning_nums, my_nums) = parse_card(line);
        let win: Vec<u32> = winning_nums.intersection(&my_nums).copied().collect();
        let points: u32 = win.len() as u32;

        if points > 0 {
            sum += 2_u32.pow(points - 1);
        }
    }
    sum
}

fn part2(input: &String) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut cards = vec![0; lines.len()];
    for i in 0..lines.len() {
        let (winning_nums, my_nums) = parse_card(lines[i]);
        cards[i] += 1;
        let wins: Vec<u32> = winning_nums.intersection(&my_nums).copied().collect();

        for win in 0..wins.len() {
            cards[i + win + 1] += cards[i];
        }
    }
    cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_test() {
        let result = part1(&INPUT.to_string());
        assert_eq!(result, 13);
    }

    #[test]
    fn part2_test() {
        let result = part2(&INPUT.to_string());
        assert_eq!(result, 30);
    }
}

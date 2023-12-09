use std::fs;

fn main() {
    let day: u32 = 9;
    println!("Solution to Day {day}");
    let input: String = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

type History = Vec<i32>;

fn parse_history(line: &str) -> History {
    line.split(" ")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn step_history(hist: History) -> History {
    let mut result: History = vec![];
    for i in 1..hist.len() {
        let diff: i32 = hist[i] - hist[i - 1];
        result.push(diff);
    }
    result
}

fn calc_hist_diffs(hist: History) -> i32 {
    let mut history: History = hist.clone();
    let mut diffs: i32 = 0;
    while !history.iter().all(|n| n == &0) {
        diffs += *history.last().unwrap();
        history = step_history(history);
    }
    diffs
}

fn part1(input: &String) -> i32 {
    let mut sum: i32 = 0;
    for line in input.lines() {
        let history: History = parse_history(line);
        sum += calc_hist_diffs(history);
    }
    sum
}

fn part2(input: &String) -> i32 {
    let mut sum: i32 = 0;
    for line in input.lines() {
        let mut history: History = parse_history(line);
        history.reverse();
        sum += calc_hist_diffs(history);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_test() {
        let result = part1(&INPUT.to_string());
        assert_eq!(result, 114);
    }

    #[test]
    fn part2_test() {
        let result = part2(&INPUT.to_string());
        assert_eq!(result, 2);
    }
}

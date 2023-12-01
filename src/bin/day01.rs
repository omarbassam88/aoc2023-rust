use std::fs;

fn main() {
    let day: u32 = 1;
    println!("Solution to Day {day}");
    let input: String = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn part1(input: &String) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        // Filter each line by digits only
        let entry: Vec<char> = line.chars().filter(|c: &char| c.is_numeric()).collect();
        let digits: String = format!("{}{}", entry[0], entry[entry.len() - 1]);
        sum += digits.parse::<u32>().unwrap();
    }
    sum
}

fn part2(input: &String) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        // matches of digits 1-9 or spelled out digits
        let mut matches: Vec<char> = vec![];
        for index in 0..line.len() {
            let slice: &str = &line[index..];
            if slice.starts_with("one") {
                matches.push('1');
            } else if slice.starts_with("two") {
                matches.push('2');
            } else if slice.starts_with("three") {
                matches.push('3')
            } else if slice.starts_with("four") {
                matches.push('4')
            } else if slice.starts_with("five") {
                matches.push('5')
            } else if slice.starts_with("six") {
                matches.push('6')
            } else if slice.starts_with("seven") {
                matches.push('7')
            } else if slice.starts_with("eight") {
                matches.push('8')
            } else if slice.starts_with("nine") {
                matches.push('9')
            } else {
                // Check if current character is a digit
                let c: char = line.chars().nth(index).unwrap();
                if c.is_digit(10) {
                    matches.push(c);
                }
            }
        }
        let digits: String = format!("{}{}", matches[0], matches[matches.len() - 1]);
        sum += digits.parse::<u32>().unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part1(&input.to_string());
        assert_eq!(result, 142);
    }

    #[test]
    fn part2_test() {
        let input: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = part2(&input.to_string());
        assert_eq!(result, 281);
    }
}

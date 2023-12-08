use std::collections::HashMap;
use std::fs;

fn main() {
    let day = 8;
    println!("Solution to Day {day}");
    let input = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn parse_map(s: &str) -> HashMap<&str, (&str, &str)> {
    let lines = s.lines();
    let mut map: HashMap<_, (_, _)> = HashMap::new();
    for line in lines {
        let mut line_iter = line.split(" = ");
        let node = line_iter.next().unwrap();
        let left_right = line_iter.next().unwrap();
        let left = &left_right[1..4];
        let right = &left_right[6..9];
        map.insert(node, (left, right));
    }

    map
}

fn part1(input: &String) -> u64 {
    let mut lines_iter = input.split("\n\n");
    let mut instructions = lines_iter.next().unwrap().chars().cycle();
    let map_str = lines_iter.next().unwrap();
    let mut current_node = "AAA";
    let goal_node = "ZZZ";

    let map = parse_map(map_str);
    let mut steps = 0;

    while current_node != goal_node {
        let current_instruction = instructions.next().unwrap();
        current_node = match current_instruction {
            'L' => map.get(current_node).unwrap().0,
            'R' => map.get(current_node).unwrap().1,
            _ => panic!("Unknown instruction"),
        };
        steps += 1;
    }

    steps
}

// Least Common Multiple of a list of numbers
pub fn lcm(nums: Vec<u64>) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums[1..].to_vec());
    a * b / gcd_of_two_numbers(a, b)
}

// Greatest Common Denominator of two numbers
fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn part2(input: &String) -> u64 {
    let mut lines_iter = input.split("\n\n");
    let instructions = lines_iter.next().unwrap().chars().cycle();
    let map_str = lines_iter.next().unwrap();
    let map = parse_map(map_str);
    let current_nodes: Vec<_> = map
        .keys()
        .filter(|n| n.chars().nth(2).unwrap() == 'A')
        .collect();

    let mut lengths = vec![];
    for node in current_nodes {
        let mut insts = instructions.clone();
        let mut current_node = node;
        let mut steps = 0;

        while !current_node.ends_with('Z') {
            let current_instruction = insts.next().unwrap();
            current_node = match current_instruction {
                'L' => &map.get(current_node.clone()).unwrap().0,
                'R' => &map.get(current_node.clone()).unwrap().1,
                _ => panic!("Unknown instruction"),
            };
            steps += 1;
        }

        lengths.push(steps);
    }
    lcm(lengths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input_1: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(&input_1.to_string());
        assert_eq!(result, 2);
        let input_2: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(&input_2.to_string());
        assert_eq!(result, 6);
    }

    #[test]
    fn part2_test() {
        let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part2(&input.to_string());
        assert_eq!(result, 6);
    }
}

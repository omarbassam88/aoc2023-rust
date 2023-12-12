use std::{collections::HashMap, fs};

fn main() {
    let day = 12;
    println!("Solution to Day {day}");
    let input = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

type Springs = Vec<char>;
type Groups = Vec<usize>;

// Inspired by https://github.com/tymscar/Advent-Of-Code/blob/master/2023/rust/src/day12/part2.rs
fn count_arrangements(
    springs: &Springs,
    groups: &Groups,
    memo: &mut HashMap<(Springs, Groups), usize>,
) -> usize {
    if springs.is_empty() {
        if groups.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    let first_spring = springs[0];
    let next_comb = count_arrangements(&springs[1..].to_vec(), groups, memo);
    match first_spring {
        '.' => next_comb,
        '#' => {
            if let Some(&curr_comb) = memo.get(&(springs.clone(), groups.clone())) {
                return curr_comb;
            }

            if groups.is_empty() {
                return 0;
            }
            let wanted_len = groups[0];
            if springs.len() < wanted_len {
                return 0;
            }

            for spring in &springs[0..wanted_len] {
                if spring == &'.' {
                    return 0;
                }
            }
            if springs.len() == wanted_len {
                if groups.len() == 1 {
                    return 1;
                }
                return 0;
            }
            if springs[wanted_len] == '#' {
                return 0;
            }

            let next_comb_next_grp = count_arrangements(
                &springs[(wanted_len + 1)..].to_vec(),
                &groups[1..].to_vec(),
                memo,
            );

            memo.insert((springs.clone(), groups.clone()), next_comb_next_grp);

            next_comb_next_grp
        }
        '?' => {
            if let Some(&curr_comb) = memo.get(&(springs.clone(), groups.clone())) {
                return curr_comb + next_comb;
            }

            if groups.is_empty() {
                return next_comb;
            }
            let wanted_len = groups[0];
            if springs.len() < wanted_len {
                return next_comb;
            }
            for spring in &springs[0..wanted_len] {
                if spring == &'.' {
                    return next_comb;
                }
            }
            if springs.len() == wanted_len {
                if groups.len() == 1 {
                    return 1 + next_comb;
                }
                return next_comb;
            }

            if springs[wanted_len] == '#' {
                return next_comb;
            }

            let next_comb_next_grp = count_arrangements(
                &springs[(wanted_len + 1)..].to_vec(),
                &groups[1..].to_vec(),
                memo,
            );

            memo.insert((springs.clone(), groups.clone()), next_comb_next_grp);

            next_comb + next_comb_next_grp
        }
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let (s, n) = line.split_once(" ").unwrap();
        let springs: Springs = s.chars().collect();
        let groups = n.split(",").map(|n| n.parse().unwrap()).collect();
        let mut memo = HashMap::new();
        sum += count_arrangements(&springs, &groups, &mut memo);
    }
    sum
}

fn part2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let (s, n) = line.split_once(" ").unwrap();
        let springs: Springs = format!("{s}?{s}?{s}?{s}?{s}").chars().collect();
        let groups = format!("{n},{n},{n},{n},{n}")
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();
        let mut memo = HashMap::new();
        sum += count_arrangements(&springs, &groups, &mut memo);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part1_test() {
        let result = part1(&INPUT);
        assert_eq!(result, 21);
    }

    #[test]
    fn part2_test() {
        let result = part2(&INPUT);
        assert_eq!(result, 525152);
    }
}

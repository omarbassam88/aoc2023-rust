use std::cmp::min;
use std::fs;
use std::iter::zip;

fn main() {
    let day = 13;
    println!("Solution to Day {day}");
    let input = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn find_mirror(lines: &Vec<&str>) -> usize {
    let mut refl = 0;
    for row in 1..lines.len() {
        let above: Vec<&str> = lines[..row].iter().copied().rev().collect();
        let below = &lines[row..];
        let min_len = min(above.len(), below.len());
        if above[0..min_len].join("\n") == below[0..min_len].join("\n") {
            refl = row;
            break;
        }
    }
    refl
}

fn part1(input: &str) -> usize {
    let patterns: Vec<&str> = input.split("\n\n").collect();
    let mut reflections = 0;
    for pattern in patterns {
        let lines: Vec<&str> = pattern.lines().collect();
        let horiz_refl = find_mirror(&lines);
        if horiz_refl > 0 {
            reflections += 100 * horiz_refl;
            continue;
        }

        // Check Vertical Reflection
        let transposed: Vec<_> = (0..lines[0].len())
            .map(|n| lines.iter().map(|l| &l[n..n + 1]).collect::<String>())
            .collect();
        let vert_refl = find_mirror(&transposed.iter().map(|s| s.as_str()).collect());
        reflections += vert_refl;
    }
    reflections
}

fn find_mirror_smudge(lines: &Vec<&str>) -> usize {
    let mut refl = 0;
    for row in 1..lines.len() {
        let above: Vec<&str> = lines[..row].iter().copied().rev().collect();
        let below = &lines[row..];
        let min_len = min(above.len(), below.len());
        let mut mismatches = 0;
        let mut possible = 0;
        for i in 0..min_len {
            if mismatches > 1 {
                break;
            };
            if above[i] != below[i] {
                mismatches += 1;
                possible = i;
            }
        }

        // only one row mismatch
        if mismatches == 1 {
            // That row only mismatches in one character
            if zip(above[possible].chars(), below[possible].chars())
                .filter(|(a, b)| a != b)
                .count()
                == 1
            {
                refl = row;
            }
        }
    }
    refl
}

fn part2(input: &str) -> usize {
    let patterns: Vec<&str> = input.split("\n\n").collect();
    let mut reflections = 0;
    for pattern in patterns {
        let lines: Vec<&str> = pattern.lines().collect();
        let horiz_refl = find_mirror_smudge(&lines);
        if horiz_refl > 0 {
            reflections += 100 * horiz_refl;
            continue;
        }

        // Check Vertical Reflection
        let transposed: Vec<_> = (0..lines[0].len())
            .map(|n| lines.iter().map(|l| &l[n..n + 1]).collect::<String>())
            .collect();
        let vert_refl = find_mirror_smudge(&transposed.iter().map(|s| s.as_str()).collect());
        reflections += vert_refl;
    }
    reflections
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part1_test() {
        let result = part1(&INPUT);
        assert_eq!(result, 405);
    }

    #[test]
    fn part2_test() {
        let result = part2(&INPUT);
        assert_eq!(result, 400);
    }
}

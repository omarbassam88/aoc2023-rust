use std::collections::HashSet;
use std::fs;

fn main() {
    let day = 11;
    println!("Solution to Day {day}");
    let input = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input, 1000000));
}

type Coord = (usize, usize);

struct Universe {
    galaxies: Vec<Coord>,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
}

fn parse_universe(input: &str) -> Universe {
    let mut empty_rows: HashSet<usize> = HashSet::new();
    // Assume all columns are empty at first
    let num_cols = input.lines().nth(0).unwrap().len();
    let mut empty_cols: HashSet<usize> = (0..num_cols).collect();
    let mut galaxies: Vec<Coord> = vec![];
    for (row, line) in input.lines().enumerate() {
        let mut empty_row = true;
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((row, col));
                // Column is not empty
                empty_cols.remove(&col);
                // Row is not empty
                empty_row = false;
            }
        }
        if empty_row {
            empty_rows.insert(row);
        }
    }

    Universe {
        galaxies,
        empty_rows,
        empty_cols,
    }
}

fn calc_paths(universe: Universe, expansion: usize) -> usize {
    let galaxies = universe.galaxies;

    let mut sum = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        // find distances
        for j in i + 1..galaxies.len() {
            let other = galaxies[j];
            let mut diff_y = galaxy.0.abs_diff(other.0);
            for r in &universe.empty_rows {
                if (r > &galaxy.0 && r < &other.0) || (r < &galaxy.0 && r > &other.0) {
                    diff_y += expansion - 1;
                }
            }
            let mut diff_x = galaxy.1.abs_diff(other.1);
            for c in &universe.empty_cols {
                if (c > &galaxy.1 && c < &other.1) || (c < &galaxy.1 && c > &other.1) {
                    diff_x += expansion - 1;
                }
            }
            let distance = diff_y + diff_x;
            sum += distance;
        }
    }

    sum
}

fn part1(input: &str) -> usize {
    let universe: Universe = parse_universe(input);
    calc_paths(universe, 2)
}

fn part2(input: &str, expansion: usize) -> usize {
    let universe: Universe = parse_universe(input);
    calc_paths(universe, expansion)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1_test() {
        let result = part1(&INPUT);
        assert_eq!(result, 374);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&INPUT, 10), 1030);
        assert_eq!(part2(&INPUT, 100), 8410);
    }
}

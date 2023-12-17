use std::{collections::HashSet, fs};

fn main() {
    let day: u32 = 16;
    println!("Solution to Day {day}");
    let input: String = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Direction {
    North,
    West,
    East,
    South,
}

#[derive(Debug, Clone)]
struct Tile {
    value: char,
    directions: HashSet<Direction>,
}

type Grid = Vec<Vec<Tile>>;
type Position = (usize, usize);

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Tile {
                    value: c,
                    directions: HashSet::new(),
                })
                .collect()
        })
        .collect()
}

fn visit(grid: &mut Grid, pos: Position, dir: Direction) -> () {
    let width = grid[0].len();
    let height = grid.len();
    let (r, c) = pos;
    // Check if passed in that position with that direction
    if grid[r][c].directions.get(&dir).is_some() {
        return;
    } else {
        grid[r][c].directions.insert(dir.clone());
    }

    match (grid[r][c].value, &dir) {
        ('|', Direction::West | Direction::East) => {
            // Go Up and Down
            if r > 0 {
                visit(grid, (r - 1, c), Direction::North)
            }
            if r < height - 1 {
                visit(grid, (r + 1, c), Direction::South)
            }
        }
        ('-', Direction::North | Direction::South) => {
            // Go Left and Right
            if c > 0 {
                visit(grid, (r, c - 1), Direction::West)
            }
            if c < width - 1 {
                visit(grid, (r, c + 1), Direction::East)
            }
        }

        ('\\', Direction::North) | ('/', Direction::South) => {
            // Go Left
            if c > 0 {
                visit(grid, (r, c - 1), Direction::West)
            }
        }
        ('\\', Direction::West) | ('/', Direction::East) => {
            // Go UP
            if r > 0 {
                visit(grid, (r - 1, c), Direction::North)
            }
        }
        ('\\', Direction::South) | ('/', Direction::North) => {
            // Go Right
            if c < width - 1 {
                visit(grid, (r, c + 1), Direction::East)
            };
        }
        ('\\', Direction::East) | ('/', Direction::West) => {
            // Go Down
            if r < height - 1 {
                visit(grid, (r + 1, c), Direction::South)
            };
        }
        ('|', Direction::North | Direction::South)
        | ('-', Direction::West | Direction::East)
        | ('.', _) => {
            // Continue in the same direction
            match dir {
                Direction::North => {
                    if r > 0 {
                        visit(grid, (r - 1, c), dir)
                    }
                }
                Direction::West => {
                    if c > 0 {
                        visit(grid, (r, c - 1), dir)
                    }
                }
                Direction::South => {
                    if r < height - 1 {
                        visit(grid, (r + 1, c), dir)
                    }
                }
                Direction::East => {
                    if c < width - 1 {
                        visit(grid, (r, c + 1), dir)
                    }
                }
            }
        }
        (_, _) => return,
    }
}

fn count_energized(grid: Grid) -> usize {
    let mut sum = 0;
    for line in grid {
        for tile in line {
            if !tile.directions.is_empty() {
                sum += 1;
            }
        }
    }
    sum
}

fn part1(input: &str) -> usize {
    let mut grid: Grid = parse_grid(input);
    visit(&mut grid, (0, 0), Direction::East);
    count_energized(grid)
}

fn part2(input: &str) -> usize {
    let grid: Grid = parse_grid(input);
    let mut max = 0;
    let last_row = grid.len() - 1;
    let last_col = grid[0].len() - 1;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            match (row, col) {
                // Top Row
                (0, c) => {
                    // Go Down
                    let mut grid_down = grid.clone();
                    visit(&mut grid_down, (0, c), Direction::South);
                    max = std::cmp::max(max, count_energized(grid_down));
                }
                // Bottom Row
                (r, c) if r == last_row => {
                    // Go UP
                    let mut grid_up = grid.clone();
                    visit(&mut grid_up, (r, c), Direction::North);
                    max = std::cmp::max(max, count_energized(grid_up));
                }
                // Left Edge
                (r, 0) => {
                    // Go Right
                    let mut grid_right = grid.clone();
                    visit(&mut grid_right, (r, 0), Direction::East);
                    max = std::cmp::max(max, count_energized(grid_right));
                }
                // Right Edge
                (r, c) if c == last_col => {
                    // Go Left
                    let mut grid_left = grid.clone();
                    visit(&mut grid_left, (r, c), Direction::West);
                    max = std::cmp::max(max, count_energized(grid_left));
                }
                (_, _) => continue,
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part1_test() {
        let result = part1(&INPUT);
        assert_eq!(result, 46);
    }

    #[test]
    fn part2_test() {
        let result = part2(&INPUT);
        assert_eq!(result, 51);
    }
}

use std::fs;

fn main() {
    let day: u32 = 10;
    println!("Solution to Day {day}");
    let input: String = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Tile {
    /// | Vertical Pipe
    Vertical,
    /// - Horizontal Pipe
    Horizontal,
    /// L
    NorthEast,
    /// J
    NorthWest,
    /// 7
    SouthEast,
    /// F
    SouthWest,
    /// . Ground
    Ground,
    /// S Start Position
    Start,
}

type Coord = (usize, usize);

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    start: Coord,
}

fn parse_grid(input: &str) -> Grid {
    let mut start = (0, 0);
    let mut tiles = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let tile: Tile = match c {
                '|' => Tile::Vertical,
                '-' => Tile::Horizontal,
                'L' => Tile::NorthEast,
                'J' => Tile::NorthWest,
                '7' => Tile::SouthEast,
                'F' => Tile::SouthWest,
                '.' => Tile::Ground,
                'S' => {
                    start = (y, x);
                    Tile::Start
                }
                _ => panic!("Unreognized Tile"),
            };
            row.push(tile);
        }
        tiles.push(row);
    }
    Grid { tiles, start }
}

fn find_loop(grid: &Grid) -> Vec<(usize, usize)> {
    let tiles = &grid.tiles;
    // Breadth First Search
    let mut visited = vec![grid.start];
    let mut queue: Vec<Coord> = vec![grid.start];

    let is_visited =
        |visited: &Vec<Coord>, row, col| visited.iter().any(|(r, c)| r == &row && c == &col);

    while !queue.is_empty() {
        let (row, col) = queue.pop().unwrap();
        let tile = tiles[row][col];
        // Can we Go UP?
        if row > 0 && !is_visited(&visited, row - 1, col) {
            match (tile, tiles[row - 1][col]) {
                (
                    Tile::Start | Tile::Vertical | Tile::NorthEast | Tile::NorthWest,
                    Tile::Vertical | Tile::SouthEast | Tile::SouthWest,
                ) => {
                    visited.push((row - 1, col));
                    queue.insert(0, (row - 1, col));
                }
                (_, _) => {}
            };
        }
        // Can we Go Down?
        if row < tiles.len() - 1 && !is_visited(&visited, row + 1, col) {
            match (tile, tiles[row + 1][col]) {
                (
                    Tile::Start | Tile::Vertical | Tile::SouthEast | Tile::SouthWest,
                    Tile::Vertical | Tile::NorthEast | Tile::NorthWest,
                ) => {
                    visited.push((row + 1, col));
                    queue.insert(0, (row + 1, col));
                }
                (_, _) => {}
            };
        }
        // Can we Go Left?
        if col > 0 && !is_visited(&visited, row, col - 1) {
            match (tile, tiles[row][col - 1]) {
                (
                    Tile::Start | Tile::Horizontal | Tile::NorthWest | Tile::SouthEast,
                    Tile::Horizontal | Tile::NorthEast | Tile::SouthWest,
                ) => {
                    visited.push((row, col - 1));
                    queue.insert(0, (row, col - 1));
                }
                (_, _) => {}
            };
        }
        // Can we Go Right?
        if col < tiles[0].len() - 1 && !is_visited(&visited, row, col + 1) {
            match (tile, tiles[row][col + 1]) {
                (
                    Tile::Start | Tile::Horizontal | Tile::NorthEast | Tile::SouthWest,
                    Tile::Horizontal | Tile::NorthWest | Tile::SouthEast,
                ) => {
                    visited.push((row, col + 1));
                    queue.insert(0, (row, col + 1));
                }
                (_, _) => {}
            };
        }
    }
    visited
}

fn part1(input: &str) -> usize {
    let grid: Grid = parse_grid(input);
    let main_loop = find_loop(&grid);
    main_loop.len().div_ceil(2)
}

fn part2(input: &str) -> usize {
    let grid = parse_grid(input);
    let main_loop = find_loop(&grid);
    let mut enclosed = vec![];
    for (y, row) in grid.tiles.iter().enumerate() {
        for x in 0..row.len() {
            // Skip if is part of the main looop
            if main_loop.iter().any(|(r, c)| r == &y && c == &x) {
                continue;
            }
            // Check Collision
            // Check Horizontal from point looking Right
            let mut collisions = vec![];
            for i in x..row.len() {
                if main_loop.iter().any(|(r, c)| r == &y && c == &i) {
                    let tile = grid.tiles[y][i];
                    let last = collisions.last().unwrap_or(&Tile::Horizontal);
                    match (last, tile) {
                        // Matching Corners represent an edge
                        // L, J or F, 7
                        (Tile::NorthEast, Tile::NorthWest) | (Tile::SouthWest, Tile::SouthEast) => {
                            collisions.pop();
                        }
                        // Non-Matching Corners act as a Vertical Collision
                        // L, 7 or F, J
                        (Tile::NorthEast, Tile::SouthEast) | (Tile::SouthWest, Tile::NorthWest) => {
                            collisions.pop();
                            collisions.push(Tile::Vertical);
                        }
                        // _, | or L or F
                        (_, Tile::Vertical | Tile::NorthEast | Tile::SouthWest) => {
                            collisions.push(tile);
                        }
                        (_, _) => {}
                    }
                }
            }
            if collisions.len() % 2 != 0 {
                enclosed.push((y, x));
            }
        }
    }
    enclosed.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input_1: &str = r".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(part1(input_1), 4);
        let input_2: &str = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(part1(input_2), 8);
    }

    #[test]
    fn part2_test() {
        let input_1: &str = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(part2(input_1), 4);
        let input_2: &str = r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!(part2(input_2), 4);
        let input_3: &str = r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(part2(input_3), 8);
    }
}

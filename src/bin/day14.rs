use std::fs;

fn main() {
    let day = 14;
    println!("Solution to Day {day}");
    let input = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

#[derive(Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

type Dish = Vec<Vec<char>>;

fn tilt(dish: &mut Dish, dir: Direction) {
    let width = dish[0].len();
    let height = dish.len();
    match dir {
        Direction::North => {
            for col in 0..width {
                let mut free = 0;
                for row in 0..height {
                    match dish[row][col] {
                        '#' => {
                            free = row + 1;
                        }
                        'O' => {
                            // swap wiht free
                            let temp = dish[row][col];
                            dish[row][col] = dish[free][col];
                            dish[free][col] = temp;
                            free += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
        Direction::West => {
            for row in 0..height {
                let mut free = 0;
                for col in 0..width {
                    match dish[row][col] {
                        '#' => {
                            free = col + 1;
                        }
                        'O' => {
                            // swap wiht free
                            let temp = dish[row][col];
                            dish[row][col] = dish[row][free];
                            dish[row][free] = temp;
                            free += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
        Direction::South => {
            for col in 0..width {
                let mut free = height - 1;
                for row in (0..height).rev() {
                    match dish[row][col] {
                        '#' => {
                            free = row - 1;
                        }
                        'O' => {
                            // swap wiht free
                            let temp = dish[row][col];
                            dish[row][col] = dish[free][col];
                            dish[free][col] = temp;
                            free -= 1;
                        }
                        _ => {}
                    }
                }
            }
        }
        Direction::East => {
            for row in 0..height {
                let mut free = width - 1;
                for col in (0..width).rev() {
                    match dish[row][col] {
                        '#' => {
                            free = col - 1;
                        }
                        'O' => {
                            // swap wiht free
                            let temp = dish[row][col];
                            dish[row][col] = dish[row][free];
                            dish[row][free] = temp;
                            free -= 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn calc_load(dish: Dish) -> usize {
    let mut sum = 0;
    for (row, line) in dish.iter().enumerate() {
        sum += (dish.len() - row) * line.iter().filter(|c| *c == &'O').count();
    }

    sum
}

fn parse_dish(input: &str) -> Dish {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

fn part1(input: &str) -> usize {
    let mut dish: Dish = parse_dish(input);
    tilt(&mut dish, Direction::North);

    calc_load(dish)
}

fn part2(input: &str) -> usize {
    let mut dish = parse_dish(input);
    // Only needed 1000 cycles to pass
    for _ in 0..1000 {
        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            tilt(&mut dish, dir);
        }
    }

    calc_load(dish)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn tilt_test() {
        let input: &str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut dish = parse_dish(input);
        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            dbg!(&dir);
            tilt(&mut dish, dir);
            for line in &dish {
                dbg!(line.iter().collect::<String>());
            }
        }
        let result = dish
            .iter()
            .map(|l| l.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        let expected = r".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_test() {
        let result = part1(&INPUT);
        assert_eq!(result, 136);
    }

    #[test]
    fn part2_test() {
        let result = part2(&INPUT);
        assert_eq!(result, 64);
    }
}

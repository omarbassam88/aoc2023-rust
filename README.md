# Advent of Code 2023

This is my solution for [Advent of code](https://adventofcode.com) for the year 2023 in [Rust](https://www.rust-lang.org).

## Usage

### Puzzle input

The input of the puzzle is unique for every user. I've included my own puzzle input in the repository. However, you should fetch your own puzzle input. To do so, I've added a `fetch.sh` script to help you with that. First you will need to get your Adent of Code session cookie from you browser's local storage and export it as an env variable as follows:

```sh
export AOC_SESSION=YOUR_AOC_SESSION_COOKIE
```

Then you can fetch the day you want:

```sh
chmod +x fetch.sh
./fetch.sh 1
```

### Testing

You can run the tests for a specific day as follows:

```sh
cargo test --bin day01
```

### Running Final Solutions

You can run the solution on the puzzle input for a specific day using the following command:

```sh
cargo run --bin day01
```

## Progress

- [x] [day01](./src/bin/day01.rs)
- [x] [day02](./src/bin/day02.rs)
- [x] [day03](./src/bin/day03.rs)
- [x] [day04](./src/bin/day04.rs)
- [x] [day05](./src/bin/day05.rs)
- [x] [day06](./src/bin/day06.rs)
- [x] [day07](./src/bin/day07.rs)
- [x] [day08](./src/bin/day08.rs)
- [x] [day09](./src/bin/day09.rs)
- [x] [day10](./src/bin/day10.rs)
- [x] [day11](./src/bin/day11.rs)
- [x] [day12](./src/bin/day12.rs)

## Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)

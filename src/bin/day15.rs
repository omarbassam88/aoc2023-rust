use std::fs;

fn main() {
    let day = 15;
    println!("Solution to Day {day}");
    let input = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn calc_hash(seq: &str) -> usize {
    let mut current_value = 0;
    for c in seq.chars() {
        // Determine the ASCII code for the current character of the string.
        let code = c as usize;
        // Increase the current value by the ASCII code you just determined.
        current_value += code;
        // Set the current value to itself multiplied by 17.
        current_value *= 17;
        // Set the current value to the remainder of dividing itself by 256.
        current_value = current_value % 256;
    }
    current_value
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    for seq in input.trim().split(",") {
        sum += calc_hash(seq);
    }
    sum
}

type Lens<'a> = (&'a str, usize);
type LensBox<'a> = Vec<Lens<'a>>;

fn part2(input: &str) -> usize {
    let mut boxes: Vec<LensBox> = vec![];
    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    for seq in input.trim().split(",") {
        match seq.split_once('=') {
            Some((label, focal_len)) => {
                let box_num = calc_hash(label);
                if let Some(index) = boxes[box_num].iter().position(|(l, _)| *l == label) {
                    boxes[box_num][index] = (label, focal_len.parse().unwrap());
                } else {
                    boxes[box_num].push((label, focal_len.parse().unwrap()));
                }
            }
            None => {
                let (label, _) = seq.split_once('-').unwrap();
                // Find if label already in a box
                if let Some(box_no) = boxes
                    .iter()
                    .position(|b| b.iter().position(|(l, _)| *l == label).is_some())
                {
                    let index = boxes[box_no].iter().position(|(l, _)| *l == label).unwrap();
                    boxes[box_no].remove(index);
                }
            }
        }
    }

    let mut sum = 0;

    for b in 0..256 {
        let bx = &boxes[b];
        for slot in 0..bx.len() {
            sum += (b + 1) * (slot + 1) * bx[slot].1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part1_test() {
        let result = part1(&INPUT);
        assert_eq!(result, 1320);
    }

    #[test]
    fn part2_test() {
        let result = part2(&INPUT);
        assert_eq!(result, 145);
    }
}

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn main() {
    let day = 7;
    println!("Solution to Day {day}");
    let input = fs::read_to_string(format!("input/{:02}.txt", day)).unwrap();
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

struct Hand {
    cards: Vec<u32>,
    bid: u32,
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            let mut result = Ordering::Less;
            for i in 0..self.cards.len() {
                if self.cards[i] == other.cards[i] {
                    continue;
                } else {
                    result = self.cards[i].cmp(&other.cards[i]);
                    break;
                }
            }
            result
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Eq for Hand {}

fn parse_hand(line: &str, joker_weekest: bool) -> Hand {
    let mut line_iter = line.split(" ");
    let cards_str = line_iter.next().unwrap();
    let cards: Vec<u32> = cards_str
        .chars()
        .map(|c| match c {
            'A' => {
                if joker_weekest {
                    13
                } else {
                    14
                }
            }
            'K' => {
                if joker_weekest {
                    12
                } else {
                    13
                }
            }
            'Q' => {
                if joker_weekest {
                    11
                } else {
                    12
                }
            }
            'J' => {
                if joker_weekest {
                    0
                } else {
                    11
                }
            }
            'T' => 10,
            _ => c.to_digit(10).unwrap(),
        })
        .collect(); // cards
    let bid: u32 = line_iter.next().unwrap().parse().unwrap();
    let mut m: HashMap<u32, u32> = HashMap::new();
    for card in &cards {
        m.entry(*card).and_modify(|c| *c += 1).or_insert(1);
    }
    let joker = m.get(&0).is_some();

    let hand_type: HandType = match m.keys().len() {
        // Five of a kind, where all five cards have the same label: AAAAA
        1 => HandType::FiveOfKind,
        2 => {
            if m.values().position(|n| n == &4).is_some() {
                // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
                if joker {
                    HandType::FiveOfKind
                } else {
                    HandType::FourOfKind
                }
            } else {
                // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
                if joker_weekest && joker {
                    HandType::FiveOfKind
                } else {
                    HandType::FullHouse
                }
            }
        }
        3 => {
            // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
            if m.values().position(|n| n == &3).is_some() {
                if joker_weekest && joker {
                    HandType::FourOfKind
                } else {
                    HandType::ThreeOfKind
                }
            } else {
                // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
                if joker_weekest && joker {
                    match m.get(&0) {
                        Some(1) => HandType::FullHouse,
                        Some(2) => HandType::FourOfKind,
                        _ => HandType::TwoPair,
                    }
                } else {
                    HandType::TwoPair
                }
            }
        }
        // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        4 => {
            if joker_weekest && joker {
                HandType::ThreeOfKind
            } else {
                HandType::OnePair
            }
        }
        // High card, where all cards' labels are distinct: 23456
        _ => {
            if joker_weekest && joker {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        }
    }; // let hand_type = match m.keys().len()

    Hand {
        cards,
        bid,
        hand_type,
    }
}

fn part1(input: &String) -> u32 {
    let mut hands: Vec<Hand> = vec![];
    for line in input.lines() {
        let card = parse_hand(line, false);
        hands.push(card);
    }

    hands.sort();
    let mut sum = 0;
    let mut rank = 1;
    for hand in hands {
        sum += hand.bid * rank;
        rank += 1;
    }
    sum
}

fn part2(input: &String) -> u32 {
    let mut hands: Vec<Hand> = vec![];
    for line in input.lines() {
        let card = parse_hand(line, true);
        hands.push(card);
    }

    hands.sort();
    let mut sum = 0;
    let mut rank = 1;
    for hand in hands {
        sum += hand.bid * rank;
        rank += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_test() {
        let result = part1(&INPUT.to_string());
        assert_eq!(result, 6440);
    }

    #[test]
    fn part2_test() {
        let result = part2(&INPUT.to_string());
        assert_eq!(result, 5905);
    }
}

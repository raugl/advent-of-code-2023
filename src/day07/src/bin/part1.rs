use std::{cmp::Ordering, collections::BTreeMap};

fn process(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let mut hands = parse(lines);
    hands.sort_unstable_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2));

    hands
        .into_iter()
        .zip(1..)
        .map(|((_hand, bid), rank)| rank * bid)
        .sum()
}

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<(Hand, i64)> {
    lines
        .map(|line| {
            let mut vals = line.split(' ');
            (
                Hand::new(vals.next().unwrap().to_owned()),
                vals.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq)]
struct Hand {
    hand_type: HandType,
    hand: String,
}

impl Hand {
    fn new(cards: String) -> Self {
        let mut map = BTreeMap::new();
        for ch in cards.chars() {
            if let Some(count) = map.get_mut(&ch) {
                *count += 1;
            } else {
                map.insert(ch, 1);
            }
        }

        use HandType::*;
        let hand_type = match map.len() {
            5 => HighCard,
            4 => OnePair,
            3 => {
                if map.into_iter().find(|(_ch, count)| *count == 3).is_some() {
                    ThreeOfAKind
                } else {
                    TwoPair
                }
            }
            2 => {
                if map.into_iter().find(|(_ch, count)| *count == 4).is_some() {
                    FourOfAKind
                } else {
                    FullHouse
                }
            }
            1 => FiveOfAKind,
            _ => panic!(
                "Unexpected number of cards in hand '{cards}': {}",
                map.len()
            ),
        };

        Self {
            hand_type,
            hand: cards,
        }
    }

    fn card_precedence(ch: char) -> usize {
        if let Some(precedence) = "23456789TJQKA".find(ch) {
            return precedence;
        }
        panic!("Unexpected card letter: {ch}")
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        for (ch1, ch2) in self.hand.chars().zip(other.hand.chars()) {
            if ch1 != ch2 {
                return Hand::card_precedence(ch1).cmp(&Hand::card_precedence(ch2));
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Result: {}", process(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        ";
        assert_eq!(process(input), 6440);
    }
}

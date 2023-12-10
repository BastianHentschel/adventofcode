use regex::Regex;

use crate::test_day;
use std::{cmp::Ordering, io::BufRead};

#[allow(unused)]
pub fn part1<R: BufRead>(input: R) -> impl ToString {
    let num_regex = Regex::new(r"\d+").unwrap();
    let mut hands = input
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let mut cards = [Card::Ace; 5];
            for (c, t) in line.chars().take(5).zip(cards.iter_mut()) {
                *t = Card::from_char(c);
            }
            let hand = Hand { cards };
            let bid = num_regex
                .find(&line[6..])
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            (hand, bid)
        })
        .collect::<Vec<_>>();
    hands.sort_unstable_by(|hand, other| hand.0.cmp(&other.0));
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum::<usize>()
}

#[allow(unused)]
pub fn part2<R: BufRead>(input: R) -> impl ToString {
    //todo!("part 2 implementation");
    0
}

test_day!(year2023, day07, "6440", "-1");

#[derive(PartialEq, Eq, Clone, Copy)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card character"),
        }
    }

    fn value(&self) -> u8 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Hand {
    fn get_type(&self) -> HandType {
        use HandType::*;
        let mut counts = [0u8; 14];
        for card in &self.cards {
            counts[(card.value() - 1) as usize] += 1;
        }
        if counts.contains(&5) {
            FiveOfAKind
        } else if counts.contains(&4) {
            FourOfAKind
        } else if counts.contains(&3) {
            if counts.contains(&2) {
                FullHouse
            } else {
                ThreeOfAKind
            }
        } else if counts.iter().filter(|&c| c == &2).count() == 2 {
            TwoPair
        } else if counts.contains(&2) {
            OnePair
        } else {
            HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.get_type().cmp(&other.get_type()) != Ordering::Equal {
            self.get_type().cmp(&other.get_type())
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

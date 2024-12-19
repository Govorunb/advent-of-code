use crate::*;

use std::{cmp::Ordering, sync::atomic::{AtomicBool, Ordering as AOrdering}};

pub struct Day7;

struct Hand {
    cards: Vec<usize>,
    rank: Type,
    bid: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Type {
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

const JOKER: usize = 0;

struct HandValue {
    rank: Type,
    #[allow(dead_code)] // debugging
    value: usize,
    second_best: Option<usize>
}

impl From<(Type, usize, Option<usize>)> for HandValue {
    fn from(t: (Type, usize, Option<usize>)) -> Self {
        Self { rank: t.0, value: t.1, second_best: t.2 }
    }
}

impl Hand {
    fn evaluate(cards: &[usize]) -> HandValue {
        let mut types = Vec::new();
        if cards.iter().all_equal() {
            return (Type::Five, cards[0], None).into();
        }
        let mut pairs: FxHashSet<usize> = FxHashSet::default();
        let mut triples: FxHashSet<usize> = FxHashSet::default();
        for &card in cards {
            let count = cards.iter().filter(|&&i| i == card).count();
            match count {
                5 => return (Type::Five, card, None).into(),
                4 => return (Type::Four, card, None).into(),
                3 => triples.insert(card),
                2 => pairs.insert(card),
                _ => false,
            };
            if count == 1 {
                types.push((Type::HighCard, card, None));
            }
        }
        let mut pairs: Vec<usize> = pairs.iter().cloned().collect();
        let triples: Vec<usize> = triples.iter().cloned().collect();
        pairs.sort_unstable();
        pairs.reverse();
        if !pairs.is_empty() {
            if !triples.is_empty() {
                return (Type::FullHouse, triples[0], Some(pairs[0])).into();
            } else if pairs.len() == 2 {
                return (Type::TwoPair, pairs[0], Some(pairs[1])).into();
            } else {
                return (Type::Pair, pairs[0], None).into();
            }
        } else if !triples.is_empty() {
            return (Type::Three, triples[0], None).into();
        }
        types.sort_unstable();
        types.reverse();
        //println!("{cards:?} {:?}", types[0]);
        types[0].into()
    }

    fn card_score(card: &char, part: Part) -> usize {
        match card {
            'T' => 10,
            'J' => match part {
                Part::One => 11,
                Part::Two => JOKER,
            },
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => card.to_digit(10).unwrap() as usize,
        }
    }
    fn compare(&self, other: &Hand) -> Ordering {
        let order = self.rank.cmp(&other.rank);
        let real = if order == Ordering::Equal {
            self.cards.cmp(&other.cards)
        } else {
            order
        };
        if order != real && DEBUG.load(AOrdering::Relaxed) {
            //println!("{:?} {:?} {:?} {real:?}", self.cards, other.cards, self.rank);
        }
        real
    }

    fn p2_rank(&self) -> Type {
        let value = Self::evaluate(&self.cards);
        let jokers = self.cards.iter()
            .filter(|&&i| i == JOKER)
            .count();
        if jokers == 0 {
            return value.rank;
        }
        let new_type = 
        match value.rank {
            Type::Five => value.rank, // 5X or 5J
            Type::Four => Type::Five, // 4X+1J -> 5X
            Type::FullHouse => Type::Five, // 3X+2J (or 3J+2X) -> 5X
            Type::Three => Type::Four, // 3J+1X+1Y (or 3X+1J+1Y) -> 4X+1Y
            Type::TwoPair => {
                // 2J+2X+1Y -> 4X+1Y
                // 2X+2Y+1J -> 3X+2Y
                if let Some(JOKER) = value.second_best {
                    Type::Four
                } else {
                    Type::FullHouse
                }
            },
            Type::Pair => Type::Three, // either 2J+1X or 2X+1J
            Type::HighCard => Type::Pair, // no brainer
        };
        if DEBUG.load(AOrdering::Relaxed) {
            println!("{:?} -> {new_type:?}", self.cards);
        }
        new_type
    }
}

static DEBUG: AtomicBool = AtomicBool::new(false);

impl Day<7> for Day7 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day7.txt");

    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let lines = input.lines();
        let mut hands = lines.map(|line| {
            let (hand_text, bid_text) = line.split_once(' ').unwrap();
            let cards = hand_text.chars().map(|c| Hand::card_score(&c, part)).collect_vec();
            let mut sorted_cards = cards.clone();
            sorted_cards.sort_unstable();
            sorted_cards.reverse();
            let rank = Hand::evaluate(&sorted_cards).rank;
            Hand {
                cards,
                rank,
                bid: bid_text.parse().unwrap(),
            }
        }).collect_vec();
        match part {
            Part::One => {
                DEBUG.store(false, AOrdering::Relaxed);
            },
            Part::Two => {
                //DEBUG.store(true, AOrdering::Relaxed);
                hands.iter_mut().for_each(|hand| hand.rank = hand.p2_rank());
            }
        }
        hands.sort_unstable_by(|h1, h2| h1.compare(h2));
        hands.iter()
            .enumerate()
            .fold(0, |acc, (i, h)| {
                acc + (i + 1) * h.bid
            })
    }
    const EXAMPLES: &'static [&'static str] = &[
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 6440),
                (Self::INPUT, 246795406),
            ],
            test_cases![
                (Self::EXAMPLES[0], 5905),
                (Self::INPUT, 249356515),
            ]
        ]
    }
}


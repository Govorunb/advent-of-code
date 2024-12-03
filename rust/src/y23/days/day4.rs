use crate::common::*;
use crate::test_cases;

pub const DAY4_EXAMPLE: &str =
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

pub struct Day4 {
}

struct Card {
    winning: Vec<usize>,
    drawn: Vec<usize>,
}

impl From<&str> for Card {
    fn from(line: &str) -> Self {
        let numbers_text = line.split_once(':').unwrap().1;

        let (winning_text, drawn_text) = numbers_text.split_once('|').unwrap();
        let mut winning = winning_text.trim()
            .split_ascii_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect_vec();
        winning.sort_unstable();
        let mut drawn = drawn_text.trim()
            .split_ascii_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect_vec();
        drawn.sort_unstable();
        Self {
            winning,
            drawn,
        }
    }
}

impl Card {
    pub fn count_won(&self) -> usize {
        self.winning
            .iter()
            .filter(|num| self.drawn.contains(num))
            .count()
    }

    pub fn score(&self) -> usize {
        let won = self.count_won();
        if won == 0 {
            0
        } else {
            usize::pow(2, won as u32 - 1)
        }
    }

    pub fn win_copies(&self) -> usize {
        self.winning.iter()
            .filter(|num| self.drawn.contains(num))
            .count()
    }
}

impl Day<4> for Day4 {
    type Output = usize;

    const INPUT: &'static str = include_str!("../Input/day4.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let cards = input.lines()
            .map_into()
            .collect_vec();
        match part {
            Part::One => {
                cards.iter()
                    .map(Card::score)
                    .sum()
            },
            Part::Two => {
                let mut copies = vec![1; cards.len()];

                for (i, this_card) in cards.iter().enumerate() {
                    let wins = this_card.win_copies();
                    let have_this_card = copies[i];
                    if wins == 0 {
                        continue;
                    }
                    for j in 1..=wins {
                        copies[i+j] += have_this_card;
                    }
                }

                copies.iter().sum()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY4_EXAMPLE, 13),
                (self.input(), 15205),
            ],
            test_cases![
                (DAY4_EXAMPLE, 30),
                (self.input(), 6189740),
            ]
        ]
    }
}

impl Default for Day4 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day4 {
    pub fn new() -> Self {
        Self {
        }
    }
}
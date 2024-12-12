use crate::test_cases;
use crate::common::*;

pub struct Day1 {
    digit_strings: FxIndexMap<&'static str, usize>,
}

impl Day<1> for Day1 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day1.txt");
    fn solve_part(&self, input: &str, part: Part) -> usize {
        input.lines()
            .map(|l| self.parse_line(l, part))
            .sum()
    }
    const EXAMPLES: &'static [&'static str] = &[
"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                ("1abc2", 12),
                ("pqr3stu8vwx", 38),
                ("a1b2c3d4e5f", 15),
                ("treb7uchet", 77),
                (Self::EXAMPLES[0], 142),
                (Self::INPUT, 54601),
            ],
            test_cases![
                ("one1", 11),
                ("two1nine", 29),
                ("eightwothree", 83),
                ("abcone2threexyz", 13),
                ("xtwone3four", 24),
                ("4nineeightseven2", 42),
                ("zoneight234", 14),
                ("7pqrstsixteen", 76),
                (Self::EXAMPLES[1], 281),
                (Self::INPUT, 54078),
            ]
        ]
    }
}

impl Day1 {
    pub fn new() -> Self {
        // bruh moment
        // From<[(K, V); N]> is only implemented for <K,V, RandomState>
        Day1 {
            digit_strings: FxIndexMap::from_iter([
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ]),
        }
    }

    fn parse_line(&self, line: &str, part: Part) -> usize {
        if line.is_empty() {
            return 0;
        }
        match part {
            Part::One => {
                let digits = line.chars().filter(char::is_ascii_digit);
                let first = digits.clone().next().unwrap_or('0');
                let last = digits.clone().nth_back(0).unwrap_or('0');
                let mut num = String::new();
                num.push(first);
                num.push(last);

                num.parse().unwrap()
            }
            Part::Two => {
                let real_digits = line
                    .char_indices()
                    .filter(|(_i, c)| c.is_ascii_digit())
                    .map(|(i, c)| (i, c.to_digit(10).unwrap() as usize))
                    .collect_vec();

                let words = self.digit_strings.iter()
                    .map(|(&k, &d)| (k,d))
                    .collect_vec();

                let first_word = words.iter()
                    .filter_map(|(key, digit)| line.match_indices(key).next().map(|(pos, _)| (pos, digit)))
                    .min_by_key(|(pos, _digit)| *pos);
                let last_word = words.iter()
                    .filter_map(|(key, digit)| line.rmatch_indices(key).next().map(|(pos, _)| (pos, digit)))
                    .max_by_key(|(pos, _digit)| *pos);

                let first = match real_digits.first() {
                    None => first_word.unwrap().1,
                    Some((pos, digit)) => match first_word {
                        None => digit,
                        Some((pos2, _)) if pos < &pos2 => digit,
                        Some((_, digit2)) => digit2,
                    },
                };

                let last = match real_digits.last() {
                    None => last_word.unwrap().1,
                    Some((pos, digit)) => match last_word {
                        None => digit,
                        Some((pos2, _)) if *pos > pos2 => digit,
                        Some((_, digit2)) => digit2,
                    },
                };
                // println!("{}: {}{} ", line, first, last);
                // println!("\t{:?} {:?} {:?}", first_word, last_word, real_digits);

                10 * first + last
            }
        }
    }
}

impl Default for Day1 {
    fn default() -> Self {
        Self::new()
    }
}

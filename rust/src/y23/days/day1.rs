use std::cmp::Ordering;
use std::sync::LazyLock;
use crate::*;

pub struct Day1;

static DIGIT_STRINGS: LazyLock<FxIndexMap<&'static str, usize>>
= LazyLock::new(|| {
    // bruh moment
    // From<[(K, V); N]> is only implemented for <K,V, RandomState>
    FxIndexMap::from_iter([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ])
});
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
    fn parse_line(&self, line: &str, part: Part) -> usize {
        if line.is_empty() {
            return 0;
        }
        match part {
            Part::One => {
                let mut digits = line.chars().filter_map(|c| c.to_digit(10));
                let first = digits.next().unwrap();
                let last = digits.next_back().unwrap_or(first);
                
                (10 * first + last) as usize
            }
            Part::Two => {
                fn find_digit(mut iter: impl Iterator<Item = (usize, char)>) -> Option<(usize, usize)> {
                    iter.find_map(|(i, c)| c.to_digit(10).map(|d| (i, d as usize)))
                }
                fn choose(num: (usize, usize), word: Option<(usize, &usize)>, cmp: Ordering) -> usize {
                    let (num_i, num_digit) = num;
                    let Some((word_i, &word_digit)) = word else {
                        return num_digit;
                    };
                    if cmp == num_i.cmp(&word_i) { num_digit } else { word_digit }
                }
                fn get_digit(line: &str, last: bool) -> usize {
                    let chars = line.char_indices();
                    
                    let num = if !last {
                        // some examples lack a numeric digit; input is guaranteed to contain at least one (as per p1)
                        find_digit(chars).unwrap_or((usize::MAX, 0))
                    } else {
                        find_digit(chars.rev()).unwrap_or((usize::MIN, 0))
                    };
                    
                    let words = DIGIT_STRINGS.iter();
                    let word = if !last {
                        words.filter_map(|(key, digit)| line.match_indices(key).next().map(|(pos, _)| (pos, digit)))
                            .min_by_key(|&(pos, _digit)| pos)
                    } else {
                        words.filter_map(|(key, digit)| line.rmatch_indices(key).next().map(|(pos, _)| (pos, digit)))
                            .max_by_key(|&(pos, _digit)| pos)
                    };
                    
                    choose(num, word, if !last {Ordering::Less} else {Ordering::Greater})
                }
                
                let [first, last] = [false, true].map(|last| get_digit(line, last));

                // println!("{}: {}{} ", line, first, last);

                10 * first + last
            }
        }
    }
}

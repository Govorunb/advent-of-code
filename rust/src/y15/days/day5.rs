use crate::test_cases;
use crate::common::*;

pub const DAY5_EXAMPLE1: &str =
"ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";

pub const DAY5_EXAMPLE2: &str =
"qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy";

pub struct Day5 {
    forbidden: Vec<String>
}

impl Day<5> for Day5 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day5.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let lines = input.lines();
        match part {
            Part::One => {
                lines
                    .filter(|l| self.is_nice_p1(l))
                    .count()
            },
            Part::Two => {
                lines
                    .filter(|l| self.is_nice_p2(l))
                    .count()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY5_EXAMPLE1, 2),
                (self.input(), 255),
            ],
            test_cases![
                (DAY5_EXAMPLE2, 2),
                // (self.input(), 0),
            ]
        ]
    }
}

impl Default for Day5 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day5 {
    pub fn new() -> Self {
        Self {
            forbidden: vec!["ab", "cd", "pq", "xy"]
                .into_iter()
                .map_into()
                .collect_vec(),
        }
    }
    
    fn is_nice_p1(&self, line: &str) -> bool {
        let mut vowels = 0;
        let mut has_double = false;
        let mut last: Option<char> = None;
        for c in line.chars() {
            if let Some(prev) = last {
                let mut s: String = prev.into();
                s.push(c);
                if self.forbidden.iter().any(|f| f.eq(&s)) {
                    return false;
                }
                if prev == c {
                    has_double = true;
                }
            }
            if "aeiou".contains(c) {
                vowels += 1;
            }
            last = Some(c);
        }
        
        has_double && vowels >= 3
    }
    
    fn is_nice_p2(&self, line: &str) -> bool {
        let mut has_wrapped = false;
        let mut has_double = false;
        let mut seeds: FxIndexMap<char, Vec<(usize, char)>> = FxIndexMap::default();
        let mut last: Option<char> = None;
        let mut before_last: Option<char> = None;
        for (i, c) in line.char_indices() {
            if !has_double {
                if let Some(prev) = last {
                    match seeds.get_mut(&prev) {
                        None => {
                            seeds.insert(prev, vec![(i,c)]);
                        }
                        Some(repeats) => {
                            has_double |= repeats.iter().any(|&r| {
                                // excludes e.g. aaa - not a double because it overlaps
                                r.0 < i-1 && r.1 == c
                            });
                            repeats.push((i,c));
                        }
                    }
                }
            }
            // enclosed by the same character on both sides, e.g. aba/aaa/exe
            // essentially line[i-2] == line[i]
            if matches!(before_last, Some(bl) if bl == c) {
                has_wrapped = true;
            }
            if has_double && has_wrapped {
                return true;
            }
            
            before_last = last;
            last = Some(c);
        }
        
        has_wrapped && has_double
    }
}
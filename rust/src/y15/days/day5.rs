use indexmap::map::Entry;
use crate::*;

pub struct Day5;

const FORBIDDEN: [(char, char); 4] = [('a','b'), ('c','d'), ('p','q'), ('x','y')];

impl Day<5> for Day5 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day5.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let lines = input.lines();
        match part {
            Part::One => {
                lines.filter(Self::is_nice_p1).count()
            },
            Part::Two => {
                lines.filter(Self::is_nice_p2).count()
            }
        }
    }
    const EXAMPLES: &'static [&'static str] = &[
"ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb",
"qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy",
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 2),
                (Self::INPUT, 255),
            ],
            test_cases![
                (Self::EXAMPLES[1], 2),
                (Self::INPUT, 55),
            ]
        ]
    }
}

impl Day5 {
    fn is_nice_p1(line: &&str) -> bool {
        let is_vowel = |c| "aeiou".contains(c);
        
        let mut vowels = 0;
        if is_vowel(line.chars().next().unwrap()) { vowels = 1; }
        let mut has_double = false;
        
        for (last, c) in line.chars().tuple_windows() {
            if FORBIDDEN.contains(&(last, c)) { return false; }
            if last == c { has_double = true; }
            if is_vowel(c) { vowels += 1; }
        }
        
        has_double && vowels >= 3
    }
    
    fn is_nice_p2(line: &&str) -> bool {
        let mut has_wrapped = false;
        let has_double = &mut false;
        let mut seeds: FxIndexMap<char, Vec<(usize, char)>> = FxIndexMap::default();

        let mut check_pairs = |i: usize, prev: char, curr: char, has: &mut bool| {
            if *has { return; }

            match seeds.entry(prev) {
                Entry::Occupied(mut seen) => {
                    *has |= seen.get().iter().any(|&(ri, rc)| ri < i - 1 && rc == curr);
                    seen.get_mut().push((i, curr));
                }
                Entry::Vacant(v) => {
                    v.insert_entry(vec![(i, curr)]);
                },
            }
        };
        
        // manually check first and second
        check_pairs(1, line.chars().nth(0).unwrap(), line.chars().nth(1).unwrap(), has_double);
        
        for (i, (a, b, c)) in line.chars().tuple_windows().enumerate() {
            // `i` is the index of `a`
            check_pairs(i+2, b, c, has_double);
            // enclosed by the same character on both sides, e.g. aba/aaa/exe
            // essentially line[i-2] == line[i]
            if a == c {
                has_wrapped = true;
            }
            
            if *has_double && has_wrapped {
                return true;
            }
        }
        
        false
    }
}
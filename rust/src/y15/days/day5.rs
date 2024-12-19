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
                lines
                    .filter(Self::is_nice_p1)
                    .count()
            },
            Part::Two => {
                lines
                    .filter(Self::is_nice_p2)
                    .count()
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
                // (Self::INPUT, 0),
            ]
        ]
    }
}

impl Day5 {
    fn is_nice_p1(line: &&str) -> bool {
        fn is_vowel(c: char) -> bool {"aeiou".contains(c)}
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
        let mut has_double = false;
        let mut seeds: FxIndexMap<char, Vec<(usize, char)>> = FxIndexMap::default();
        
        let mut check_double = move |i, prev: char, curr: char| {
            if has_double || prev != curr {return;}
            
            match seeds.entry(prev) {
                Entry::Occupied(mut repeats) => {
                    repeats.get_mut().push((i, curr));
                    has_double = repeats.get().iter().any(|&(ri, rc)| ri < i-1 && rc == curr);
                },
                Entry::Vacant(v) => {
                    v.insert(vec![(i, curr)]);
                },
            }
        };
        
        // manually check first and second
        check_double(1, line.chars().nth(0).unwrap(), line.chars().nth(1).unwrap());
        
        for (i, (a, b, c)) in line.chars().tuple_windows().enumerate() {
            check_double(i, b, c);
            // enclosed by the same character on both sides, e.g. aba/aaa/exe
            // essentially line[i-2] == line[i]
            if a == c {
                has_wrapped = true;
            }
            
            if has_double && has_wrapped {
                return true;
            }
        }
        
        false
    }
}
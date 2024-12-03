use crate::test_cases;
use crate::common::*;


pub const DAY8_INPUT: &str = include_str!("../Input/day8.txt");
pub const DAY8_EXAMPLE1: &str =
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
pub const DAY8_EXAMPLE2: &str =
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
pub const DAY8_EXAMPLE3: &str =
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

pub struct Day8 {
    
}

#[derive(Clone, Debug, PartialEq, Default)]
struct Head<'a> {
    curr: &'a str,
    first_count: usize,
    first_finished: bool,
    cycle_count: usize,
    cycle_finished: bool,
}

impl Head<'_> {
    fn next(&mut self, part: Part) {
        if !self.cycle_finished {
            self.cycle_count += 1;
            if self.at_end(part) {
                self.cycle_count -= 1;
                self.cycle_finished = true;
            }
        }
    }
    fn at_end(&self, part: Part) -> bool {
        match part {
            Part::One => self.curr == "ZZZ",
            Part::Two => self.curr.ends_with('Z'),
        }
    }
}

impl Day<8> for Day8 {
    type Output = usize;
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let instructions = input.lines().next().unwrap();
        let graph: FxHashMap<&str, (&str, &str)> = input.lines().skip(2).map(|l| {
            let (from, to) = l.split_once(" = ").unwrap();
            let (from, to) = (from.trim(), to.trim());
            let (to_l, to_r) = to.split_once(',').unwrap();
            let (to_l, to_r) = (to_l.trim_matches(|c: char| c == ' ' || c == '('), to_r.trim_matches(|c: char| c == ' ' || c == ')'));

            (from, (to_l, to_r))
        }).collect();
        let mut heads = 
            graph.keys().filter(|&&k| match part {
                Part::One => k == "AAA",
                Part::Two => k.ends_with('A'),
            }).map(|&k| Head {
                curr: k,
                ..Head::default()
            }).collect_vec();
        for instr in instructions.chars().cycle() {
            for head in &mut heads {
                let curr = head.curr;
                let roads = graph.get(curr).unwrap_or_else(|| panic!("{curr:?}"));
                let next = match instr {
                    'L' => roads.0,
                    'R' => roads.1,
                    _ => unreachable!()
                };
                head.next(part);
                head.curr = next;
            }
            if heads.iter().all(|head| head.cycle_finished) {
                break;
            }
        }
        let cycle_counts = heads.iter().map(|head| head.cycle_count).collect_vec();
        cycle_counts.iter()
            .cloned()
            .reduce(|acc, item| num::Integer::lcm(&acc, &item))
            .unwrap()
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY8_EXAMPLE1, 2),
                (DAY8_EXAMPLE2, 6),
                (DAY8_INPUT, 13301),
            ],
            test_cases![
                (DAY8_EXAMPLE1, 2),
                (DAY8_EXAMPLE2, 6),
                (DAY8_EXAMPLE3, 6),
                (DAY8_INPUT, 7309459565207),
            ]
        ]
    }
}

impl Default for Day8 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day8 {
    pub fn new() -> Self {
        Self {
        }
    }
}
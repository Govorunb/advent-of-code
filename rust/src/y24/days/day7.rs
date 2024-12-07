use crate::*;

pub const DAY7_EXAMPLE: &str =
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

pub struct Day7 {
    
}

#[derive(Debug, Clone)]
struct Equation {
    total: usize,
    elements: Vec<usize>,
}

impl Day<7> for Day7 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day7.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let lines = input.lines();
        let equations = lines.map(|l| {
            let (total_s, el_s) = l.split_once(':').unwrap();
            let total = total_s.parse::<usize>().unwrap();
            let els = el_s.split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();
            Equation {total, elements: els}
        }).collect_vec();
        match part {
            Part::One => {
                equations.iter()
                    .filter(|&eqn| Self::search(eqn.clone(), false))
                    .map(|eq| eq.total)
                    .sum()
            },
            Part::Two => {
                equations.iter()
                    .filter(|&eqn| Self::search(eqn.clone(), true))
                    .map(|eq| eq.total)
                    .sum()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY7_EXAMPLE, 3749),
                (self.input(), 850435817339),
            ],
            test_cases![
                (DAY7_EXAMPLE, 11387),
                (self.input(), 104824810233437),
            ]
        ]
    }
}

impl Default for Day7 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day7 {
    pub fn new() -> Self {
        Self {
        }
    }
    
    fn search(eqtn: Equation, concat: bool) -> bool {
        if eqtn.elements.len() == 1 {
            return eqtn.elements[0] == eqtn.total
        }
        
        let mut clone = eqtn.clone();
        let first = clone.elements.remove(0);
        let mut plus = clone.clone();
        let mut mul = clone.clone();
        let mut conc = clone.clone();
        plus.elements[0] += first;
        mul.elements[0] *= first;
        conc.elements[0] = format!("{}{}", first, conc.elements[0]).parse().unwrap();
        
        Self::search(plus, concat) || Self::search(mul, concat)
        || (concat && Self::search(conc, concat))
    }
}
use num::Integer;
use crate::*;

pub struct Day13 {
    
}
#[derive(Debug, Clone)]
struct Machine {
    a: Vector2,
    b: Vector2,
    prize: Vector2,
}
impl Day<13> for Day13 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day13.txt");

    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let regex = Regex::new(
r#"Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)
Button B: X\+(?<bx>\d+), Y\+(?<by>\d+)
Prize: X=(?<px>\d+), Y=(?<py>\d+)"#).unwrap();
        let mut machines = regex.captures_iter(input)
            .map(|c| {
                Machine {
                    a: c.vec2("ax", "ay"),
                    b: c.vec2("bx", "by"),
                    prize: c.vec2("px", "py"),
                }
            }).collect_vec();
        match part {
            Part::One => {
                machines.iter()
                    .filter_map(|m| m.cheapest_win(100).map(|c| c as usize))
                    .sum()
            },
            Part::Two => {
                for m in &mut machines {
                    m.prize += Vector2 {x: 10000000000000, y: 10000000000000};
                }
                machines.iter()
                    .filter_map(|m| m.cheapest_win(isize::MAX).map(|c| c as usize))
                    .sum()
            }
        }
    }
    const EXAMPLES: &'static [&'static str] = &[
"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 480),
                (Self::INPUT, 36250),
            ],
            test_cases![
                (Self::EXAMPLES[0], 875318608908), // not given
                (Self::INPUT, 83232379451012),
            ]
        ]
    }
}

impl Default for Day13 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day13 {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Machine {
    const A_COST: isize = 3;
    const B_COST: isize = 1;
    fn cheapest_win(&self, max_any_button: isize) -> Option<isize> {
        // TLDR: https://www.desmos.com/calculator/f9lkmedn8q
        
        // essentially two linear equations of the form Aa + Bb = P, where:
        // - a and b are the numbers of A and B presses respectively
        // - A (and B) are button A (or B)'s X or Y component
        // - P is the prize's X or Y component
        
        // just iterating over `a` and calculating b = (P - Aa) / B
        // (where calculations for both X and Y must agree) was enough for part 1
        // but for part 2 we need to actually solve the system of equations
        // e.g. from the first machine in the example:
        // (1) 94a+22b = 8400
        // (2) 34a+67b = 5400
        // multiply both sides of (2) to match its `a` coefficient with (1):
        // (1) 94a+22b = 8400
        // (2) 94a+67*(94/34)b = 5400*(94/34)
        // subtract (1) from (2):
        // ((67*94/34) - 22)b = 5400*(94/34) - 8400
        // common base of 34 (it then cancels out):
        // (67*94 - 22*34)b/34 = (5400*94 - 8400*34)/34
        // (6298-748)b = (507600-285600)
        // 5550b = 222000 => b = 40 (here, if the number is non-integer/out of bounds/etc, we can break)
        // 94a+22*40 = 8400 => a = (8400-880)/94 = 80
        
        // more generalized, it looks like this:
        // (B2*(A1/A2) - B1)b = P2*(A1/A2) - P1
        // ((B2*A1 - B1*A2) / A2)b = (P2*A1 - P1*A2) / A2
        // (B2*A1 - B1*A2)b = (P2*A1 - P1*A2)
        // b = (P2*A1 - P1*A2) / (B2*A1 - B1*A2)
        // a = (P1-(B1)b) / A1
        
        #[allow(non_snake_case)]
        let Machine {a: A, b: B, prize: P} = self;
        
        let part1 = P.y*A.x - P.x*A.y;
        let part2 = B.y*A.x - B.x*A.y;
        let (b, brem) = part1.div_rem(&part2);
        if brem != 0 || b > max_any_button {return None}
        
        // Aa+Bb=P => a=(P-Bb)/A
        let (a, arem) = (P.x-(B.x*b)).div_rem(&A.x);
        if arem != 0 || a > max_any_button {return None}

        debug_assert_eq!(self.a * a + self.b * b, self.prize, "uh oh!");
        
        // mathematically there can only be one solution (i think)
        let cost = Self::A_COST * a + Self::B_COST * b;
        Some(cost)
    }
}
use num::Integer;
use crate::*;

pub struct Day13;
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
        
        let eqn1 = LinearEquation {
            a: self.a.x,
            b: self.b.x,
            c: self.prize.x
        };
        let eqn2 = LinearEquation {
            a: self.a.y,
            b: self.b.y,
            c: self.prize.y
        };
        
        let (a,b) = solve_system_2var(&[eqn1, eqn2])?;
        if b > max_any_button || a > max_any_button {return None}

        debug_assert_eq!(self.a * a + self.b * b, self.prize, "uh oh!");
        
        // mathematically there can only be one solution (i think)
        let cost = Self::A_COST * a + Self::B_COST * b;
        Some(cost)
    }
}
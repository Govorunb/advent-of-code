use crate::test_cases;
use crate::common::*;

pub struct Day2 {
}

struct Box(u32, u32, u32);

impl Day<2> for Day2 {
    type Output = u32;
    const INPUT: &'static str = include_str!("../Input/day2.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let lines = input.lines();
        let boxes = lines.map(Day2::parse);
        match part {
            Part::One => {
                boxes
                .map(|b| {
                    let Box(l, w, h) = b;
                    let areas = [l * w, w * h, h * l];
                    let smallest = areas.iter().min().unwrap();
                    2 * areas.iter().sum::<u32>() + smallest
                })
                .sum::<u32>()
            },
            Part::Two => {
                boxes
                .map(|b| {
                    let Box(l, w, h) = b;
                    let sides = [l,w,h];
                    let largest_i = sides.iter().enumerate().max_by_key(|&(_, a)| a).unwrap().0;
                    let ribbon_face = sides.iter().enumerate().filter(|&(i, _)| i != largest_i)
                        .map(|(_, &a)| a)
                        .collect::<Vec<_>>();
                    let ribbon = 2 * ribbon_face.iter().sum::<u32>();
                    let bow = l*w*h;

                    ribbon + bow
                })
                .sum::<u32>()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                ("2x3x4", 58),
                ("1x1x10", 43)
            ],
            test_cases![
                ("2x3x4", 34),
                ("1x1x10", 14),
            ]
        ]
    }
}

impl Default for Day2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day2 {
    pub fn new() -> Self {
        Day2 {
        }
    }

    fn parse(line: &str) -> Box {
        let parts = line.split('x')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        Box(parts[0], parts[1], parts[2])
    }
}
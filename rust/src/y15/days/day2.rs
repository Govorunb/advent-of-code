use crate::*;

pub struct Day2;

struct Box(usize, usize, usize);

impl Day<2> for Day2 {
    type Output = usize;
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
                    2 * areas.into_iter().sum::<usize>() + smallest
                })
                .sum()
            },
            Part::Two => {
                boxes
                .map(|b| {
                    let Box(l, w, h) = b;
                    let sides = [l,w,h];
                    let largest_i = sides.iter().enumerate()
                        .max_by_key(|&(_, a)| a)
                        .unwrap().0;
                    let ribbon: usize = sides.iter().enumerate()
                        .filter_map(|(i, &a)| (i != largest_i).then_some(a))
                        .sum();
                    let bow = l*w*h;

                    2*ribbon + bow
                }).sum()
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

impl Day2 {
    fn parse(line: &str) -> Box {
        let parts = line.split('x')
            .map(|s| s.parse().unwrap())
            .collect_vec();

        Box(parts[0], parts[1], parts[2])
    }
}
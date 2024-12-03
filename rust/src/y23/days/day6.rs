use crate::test_cases;
use crate::common::*;

pub const DAY6_INPUT: &str = include_str!("../Input/day6.txt");
pub const DAY6_EXAMPLE: &str =
"Time:      7  15   30
Distance:  9  40  200";

pub struct Day6 {
}

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn min_to_beat(&self) -> usize {
        let start = self.distance / self.time;
        (start..self.time)
            .find(|t| t * (self.time - t) > self.distance)
            .unwrap()
    }
}

impl Day<6> for Day6 {
    type Output = usize;
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        match part {
            Part::One => {
                let (times, distances) = input.lines()
                    .map(|line| line.split_ascii_whitespace().skip(1).map(|num| num.parse().unwrap()))
                    .next_tuple()
                    .unwrap();
                times.zip_eq(distances)
                    .map(|(time, distance)| Race { time, distance })
                    .map(|race| {
                        let min = race.min_to_beat();
                        let max = race.time - min;
                        // you can do (min..=max).size_hint().0
                        // but ExactSizeIterator isn't a thing on inclusive ranges. ok
                        (min..max+1).len()
                    })
                    .reduce(|acc, o| acc * o)
                    .unwrap()
            },
            Part::Two => {
                // let min = race.min_to_beat();
                // let max = race.time - min;
                // return (min..max+1).len();
                let (time, distance) = input.lines()
                    .map(|line| line.chars().filter(char::is_ascii_digit))
                    .map(|digits| digits.collect::<String>().parse::<f64>().unwrap())
                    .next_tuple()
                    .unwrap();
                let det = (time*time - 4.*distance).sqrt();
                let free_time_l = (time - det) * 0.5;
                let free_time_r = (time + det) * 0.5;
                let free_time = free_time_r.ceil() - free_time_l.floor() - 1.;
                free_time.floor() as usize
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY6_EXAMPLE, 288),
                (DAY6_INPUT, 345015),
            ],
            test_cases![
                (DAY6_EXAMPLE, 71503),
                (DAY6_INPUT, 42588603),
                ("Time: 100\nDistance: 1000", 77),
            ]
        ]
    }
}

impl Default for Day6 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day6 {
    pub fn new() -> Self {
        Self {
        }
    }
}
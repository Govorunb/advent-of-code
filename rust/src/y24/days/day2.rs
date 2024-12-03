#![allow(dead_code)]

use crate::test_cases;
use crate::common::*;

pub const DAY2_INPUT: &str = include_str!("../Input/day2.txt");
pub const DAY2_EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

pub struct Day2 {}

#[derive(Debug, Clone)]
struct Report {
    levels: Vec<usize>
}

impl Day<2> for Day2 {
    type Output = usize;
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let lines = input.lines();
        let tolerant = match part {
            Part::One => false,
            Part::Two => true,
        };
        let reports = lines
            .map(Report::parse)
            .collect_vec();

        let safeties = reports.into_iter()
            .map(|r| r.safety(tolerant))
            .collect_vec();
        safeties.into_iter()
            .filter(|&s| s)
            .count()
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY2_EXAMPLE, 2),
                (DAY2_INPUT, 371),
            ],
            test_cases![
                (DAY2_EXAMPLE, 4),
                // (DAY2_INPUT, 0),
            ],
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
        Self {}
    }
}

impl Report {
    pub fn parse(line: &str) -> Self {
        let levels = line
            .split(" ")
            .map(|c| c.parse::<usize>().unwrap())
            .collect_vec();
        Report { levels }
    }
    pub fn safety(&self, tolerant: bool) -> bool {
        let diffs = self.levels
            .windows(2)
            .map(|w| w[1] as isize - w[0] as isize)
            .collect_vec();
        let tolerance = if tolerant {1} else {0};
        let directions = diffs.iter()
            .map(|&d| d > 0)
            .counts();
        let dominant_direction = directions.values().max().unwrap();
        let direction_faults = self.levels.len() - 1 - dominant_direction;
        let stay_faults = diffs.iter()
            .filter(|&&d| d.abs() < 1)
            .count();
        let leap_faults = diffs.iter()
            .filter(|&&d| d.abs() > 3)
            .count();
        
        if direction_faults + stay_faults + leap_faults == 0 {
            return true;
        }
        
        // println!("{:?} faulty ({:?},{:?},{:?})", self.levels, direction_faults, stay_faults, leap_faults);
        
        if tolerance == 0 || direction_faults > tolerance || stay_faults > tolerance || leap_faults > tolerance {
            return false;
        }
        // no point getting clever about it
        for omit in 0..self.levels.len() {
            let mut levels = self.levels.clone();
            levels.remove(omit);
            let report = Report { levels };
            if report.safety(false) {
                // println!("{:?} saved by omitting {:?}th ({:?}) -> into {:?}", self.levels, omit, self.levels[omit], report.levels);
                return true;
            }
        }
        // println!("{:?} not saveable", self.levels);
        false
    }
}

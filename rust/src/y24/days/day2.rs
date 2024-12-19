use crate::*;
pub struct Day2;

#[derive(Debug, Clone)]
struct Report {
    levels: Vec<isize>
}

impl Day<2> for Day2 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day2.txt");

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
    const EXAMPLES: &'static [&'static str] = &[
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 2),
                (Self::INPUT, 371),
            ],
            test_cases![
                (Self::EXAMPLES[0], 4),
                (Self::INPUT, 426),
            ],
        ]
    }
}


impl Report {
    pub fn parse(line: &str) -> Self {
        let levels = line
            .split(" ")
            .map(|c| c.parse::<isize>().unwrap())
            .collect_vec();
        Report { levels }
    }
    pub fn safety(&self, tolerant: bool) -> bool {
        let diffs = self.levels.iter()
            .tuple_windows()
            .map(|(a,b)| b-a)
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
        
        if tolerance == 0 || direction_faults > tolerance || stay_faults > tolerance || leap_faults > tolerance {
            return false;
        }
        
        // println!("{:?} faulty ({:?},{:?},{:?})", self.levels, direction_faults, stay_faults, leap_faults);
        
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

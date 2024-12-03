use super::common::*;
use stopwatch::Stopwatch;
pub use crate::test_cases;

mod day1; pub use day1::*;
// mod day2; pub use day2::*;
// mod day3; pub use day3::*;
// mod day4; pub use day4::*;
// mod day5; pub use day5::*;
// mod day6; pub use day6::*;
// mod day7; pub use day7::*;
// mod day8; pub use day8::*;
// mod day9; pub use day9::*;
// mod day10; pub use day10::*;
// mod day11; pub use day11::*;
// mod day12; pub use day12::*;
// mod day13; pub use day13::*;
// mod day14; pub use day14::*;
// mod day15; pub use day15::*;
// mod day16; pub use day16::*;
// mod day17; pub use day17::*;
// mod day18; pub use day18::*;
// mod day19; pub use day19::*;
// mod day20; pub use day20::*;
// mod day21; pub use day21::*;
// mod day22; pub use day22::*;
// mod day23; pub use day23::*;
// mod day24; pub use day24::*;
// mod day25; pub use day25::*;

#[derive(Debug, Clone, Copy, Default)]
pub enum Part
{
    #[default]
    One = 1,
    Two = 2
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Part::One => 1,
            Part::Two => 2
        })
    }
}

trait TestCase<TOutput> {
    fn input(&self) -> &str;
    fn expected(&self) -> TOutput;
}
pub struct TestCaseImpl<T: Display>(pub &'static str, pub T);

impl<T: Display + Clone> From<(&'static str, T)> for TestCaseImpl<T> {
    fn from(t: (&'static str, T)) -> Self {
        Self(t.0, t.1)
    }
}

impl<T: Display + std::fmt::Debug + Clone> TestCase<T> for TestCaseImpl<T> {
    fn input(&self) -> &str { self.0 }
    fn expected(&self) -> T { self.1.clone() }
}

pub trait Day<const DAY: u8>
{
    type Output: Display + PartialEq + std::fmt::Debug + Clone;
    #[allow(private_bounds)] // the whole point is sealing it
    type TestCase: TestCase<Self::Output> = TestCaseImpl<Self::Output>;
    fn day(&self) -> u8 {DAY}
    fn solve(&self, input: &str) {
        println!("day {}", self.day());
        let mut sw = Stopwatch::start_new();
        for part in [Part::One, Part::Two] {
            sw.restart();
            let result = self.solve_part(input, part);
            let time = sw.elapsed().as_micros();
            print!("\tpart {part:?}: {result}");
            println!(" (took {time}us)");
        }
    }
    fn solve_part(&self, input: &str, part: Part) -> Self::Output;

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2];
    fn test(&self, part: Option<Part>) {
        let [p1, p2] = self.test_cases();
        let sw = Stopwatch::start_new();
        if let Some(part) = part {
            self.test_part(part, match part { Part::One => p1, Part::Two => p2 });
            println!("day {DAY} part {part} tests passed (took {}us)", sw.elapsed().as_micros());
        } else {
            self.test_part(Part::One, p1);
            self.test_part(Part::Two, p2);
            println!("day {DAY} tests passed (took {}us)", sw.elapsed().as_micros());
        }
    }
    fn test_part(&self, part: Part, test_cases: Vec<Self::TestCase>) {
        for (i, case) in test_cases.into_iter().enumerate() {
            let (input, expected) = (case.input(), case.expected());
            let got = self.solve_part(input, part);
            assert_eq!(expected, got, "d{DAY} p{part} case {} - expected {expected}, got {got}", i+1);
        }
    }
}
#[macro_export]
macro_rules! test_cases {
    ($(($input:expr => $expected:expr)),* $(,)?) => {
        vec![$(TestCaseImpl($input, $expected),)*]
    };
    ($(($input:expr, $expected:expr)),* $(,)?) => {
        vec![$(TestCaseImpl($input, $expected),)*]
    }
}
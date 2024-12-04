mod grid; pub use grid::*;
mod iter; pub use iter::*;
mod direction; pub use direction::*;
mod rect_iter; pub use rect_iter::*;
mod point; pub use point::*;
mod rect; pub use rect::*;

pub use itertools::Itertools;
pub use rustc_hash::{FxHashMap, FxHashSet};
pub use std::fmt::Display;
pub use rayon::prelude::*;
pub use regex::{Regex, Captures};

pub type FxIndexMap<K,V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;
pub type FxIndexSet<T> = IndexSet<T, BuildHasherDefault<FxHasher>>;

use std::hash::BuildHasherDefault;
use rustc_hash::FxHasher;
use indexmap::{IndexMap, IndexSet};
use stopwatch::Stopwatch;

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
    const INPUT: &'static str;
    #[allow(private_bounds)] // the whole point is sealing it
    type TestCase: TestCase<Self::Output> = TestCaseImpl<Self::Output>;
    fn day(&self) -> u8 {DAY}
    fn input(&self) -> &'static str {Self::INPUT}
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

#[macro_export]
macro_rules! aoc_year {
    () => {
        // advent_of_code::y24 -> 2024
        "20".to_string() + &module_path!().split("::").skip(1).next().unwrap()[1..]
    }
}
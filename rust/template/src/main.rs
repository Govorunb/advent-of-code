#![feature(trait_alias)]
#![feature(associated_type_defaults)]
#![feature(ascii_char)]
#![cfg(target_pointer_width = "64")]
pub mod common;
pub mod days;

use crate::days::*;

fn main() {
    let d1 = Day1::new();
    d1.test(None);
    d1.solve(DAY1_INPUT);
}

#[test]
fn test_all_days() {
    let sw = stopwatch::Stopwatch::start_new();
    Day1::new().test(None);
    // Day2::new().test(None);
    // Day3::new().test(None);
    // Day4::new().test(None);
    // Day5::new().test(None);
    // Day6::new().test(None);
    // Day7::new().test(None);
    // Day8::new().test(None);
    // Day9::new().test(None);
    // Day10::new().test(None);
    // Day11::new().test(None);
    // Day12::new().test(None);
    // Day13::new().test(None);
    // Day14::new().test(None);
    // Day15::new().test(None);
    // Day16::new().test(None);
    // Day17::new().test(None);
    // Day18::new().test(None);
    // Day19::new().test(None);
    println!("tests took {}us", sw.elapsed().as_micros());
}

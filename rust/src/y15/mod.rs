#![cfg(target_pointer_width = "64")]

pub mod days;
use days::*;
use crate::common::*;

#[test]
pub fn test_all_days() {
    let sw = stopwatch::Stopwatch::start_new();
    Day1::new().test(None);
    Day2::new().test(None);
    Day3::new().test(None);
    Day4::new().test(None);
    println!("2015 tests took {}us", sw.elapsed().as_micros());
}

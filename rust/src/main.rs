#![feature(trait_alias)]
#![feature(associated_type_defaults)]
#![feature(ascii_char)]
#![allow(dead_code)]
#![cfg(target_pointer_width = "64")]
pub mod common;
mod y15;
mod y23;
mod y24;
// use y15::days::*;
// use y23::days::*;
use y24::days::*;
use common::*;

fn main() {
    let d2 = Day2::new();
    d2.test(None);
    d2.solve(DAY2_INPUT);
}

#[test]
fn test_all_years() {
    let sw = stopwatch::Stopwatch::start_new();
    y15::test_all_days();
    y23::test_all_days();
    y24::test_all_days();
    println!("all tests took {}us", sw.elapsed().as_micros());
}

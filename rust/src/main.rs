#![feature(trait_alias)]
#![feature(associated_type_defaults)]
#![feature(ascii_char)]
#![feature(is_none_or)]
#![feature(ascii_char_variants)]
#![feature(let_chains)]
#![feature(is_sorted)]
#![feature(iter_array_chunks)]
#![allow(dead_code)]
#![cfg(target_pointer_width = "64")]
pub mod common;
mod y15;
mod y23;
mod y24;
pub use common::*;

fn main() {
    let day = y24::days::Day9::new();
    day.test(None);
    day.solve(day.input());
}

#[test]
fn test_all_years() {
    let sw = stopwatch::Stopwatch::start_new();
    y15::test::test_all_days();
    y23::test::test_all_days();
    y24::test::test_all_days();
    println!("all tests took {}us", sw.elapsed().as_micros());
}

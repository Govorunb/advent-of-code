#![cfg(target_pointer_width = "64")]
#![allow(dead_code)]
#![allow(mismatched_lifetime_syntaxes)] // i shouldn't need to google the error text to find out the linter code

#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(associated_type_defaults)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(coroutines)]
#![feature(int_roundings)]
#![feature(iter_array_chunks)]
#![feature(iter_from_coroutine)]
#![feature(lazy_get)]
#![feature(trait_alias)]
#![feature(iter_map_windows)]
#![feature(slice_split_once)]

pub mod common; pub use common::*;
mod y15; mod y16;
mod y23; mod y24; mod y25;

fn main() {
    let day = y25::days::Day9;
    day.test(None);
    day.solve(day.input());
}

#[test]
fn test_all_years() {
    let sw = simple_stopwatch::Stopwatch::start_new();
    y15::test::test_all_days();
    y16::test::test_all_days();
    y23::test::test_all_days();
    y24::test::test_all_days();
    y25::test::test_all_days();
    println!("all tests took {}us", sw.us());
}

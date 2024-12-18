#![feature(trait_alias)]
#![feature(associated_type_defaults)]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(let_chains)]
#![feature(iter_array_chunks)]
#![feature(generic_arg_infer)]
#![feature(int_roundings)]
#![feature(const_trait_impl)]
#![feature(coroutines)]
#![feature(iter_from_coroutine)]
#![allow(dead_code)]
#![cfg(target_pointer_width = "64")]
pub mod common; pub use common::*;
mod y15; mod y23; mod y24;

fn main() {
    let day = y24::days::Day18::new();
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

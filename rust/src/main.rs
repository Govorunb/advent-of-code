#![cfg(target_pointer_width = "64")]
#![allow(dead_code)]

#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(associated_type_defaults)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(coroutines)]
#![feature(generic_arg_infer)]
#![feature(int_roundings)]
#![feature(iter_array_chunks)]
#![feature(iter_from_coroutine)]
#![feature(lazy_get)]
#![feature(let_chains)]
#![feature(trait_alias)]

pub mod common; pub use common::*;
mod y15; mod y23; mod y24;

fn main() {
    let day = y24::days::Day20;
    day.test(None);
    day.solve(day.input());
}

#[test]
fn test_all_years() {
    let sw = simple_stopwatch::Stopwatch::start_new();
    y15::test::test_all_days();
    y23::test::test_all_days();
    y24::test::test_all_days();
    println!("all tests took {}us", sw.us());
}

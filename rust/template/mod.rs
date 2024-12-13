#![cfg(target_pointer_width = "64")]
pub mod days;

#[cfg(test)]
pub mod test {
    use super::days::*;
    use crate::*;
    
    pub fn test_all_days() {
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
        // Day20::new().test(None);
        // Day21::new().test(None);
        // Day22::new().test(None);
        // Day23::new().test(None);
        // Day24::new().test(None);
        // Day25::new().test(None);
        println!("{} tests took {}us", aoc_year!(), sw.elapsed().as_micros());

    }
}

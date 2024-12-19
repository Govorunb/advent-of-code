#![cfg(target_pointer_width = "64")]
pub mod days;

#[cfg(test)]
pub mod test {
    use super::days::*;
    use crate::*;

    pub fn test_all_days() {
        let year = aoc_year!();
        println!("running {year} tests");

        let sw = stopwatch::Stopwatch::start_new();
        Day1.test(None);
        Day2.test(None);
        Day3.test(None);
        Day4.test(None);
        Day5.test(None);
        // Day6.test(None); // slow
        Day7.test(None);
        Day8.test(None);
        Day9.test(None);
        Day10.test(None);
        Day11.test(None);
        Day12.test(None);
        Day13.test(None);
        Day14.test(None);
        Day15.test(None);
        Day16.test(None);
        Day17.test(None);
        Day18.test(None);
        Day19.test(None);
        Day20.test(None);
        // Day21.test(None);
        // Day22.test(None);
        // Day23.test(None);
        // Day24.test(None);
        // Day25.test(None);
        
        println!("{year} tests took {}us", sw.elapsed().as_micros());
    }
}

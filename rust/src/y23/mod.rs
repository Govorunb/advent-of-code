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
        Day1::new().test(None);
        Day2::new().test(None);
        Day3::new().test(None);
        Day4::new().test(None);
        Day5::new().test(None);
        Day6::new().test(None);
        Day7::new().test(None);
        Day8::new().test(None);
        Day9::new().test(None);
        Day10::new().test(None);
        Day11::new().test(None);
        // Day12::new().test(None); // i give up
        Day13::new().test(None);
        Day14::new().test(None);
        Day15::new().test(None);
        Day16::new().test(None);
        // Day17::new().test(None); // wowie graph problems yippee i love reimplementing search algorithms
        // Day18::new().test(None); // this one is a grid issue (aka skill issue) on my part
        Day19::new().test(None);

        println!("{year} tests took {}us", sw.elapsed().as_micros());
    }
}

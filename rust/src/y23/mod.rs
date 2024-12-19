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
        Day6.test(None);
        Day7.test(None);
        Day8.test(None);
        Day9.test(None);
        Day10.test(None);
        Day11.test(None);
        // Day12.test(None); // i give up
        Day13.test(None);
        Day14.test(None);
        Day15.test(None);
        Day16.test(None);
        // Day17.test(None); // wowie graph problems yippee i love reimplementing search algorithms
        // Day18.test(None); // this one is a grid issue (aka skill issue) on my part
        Day19.test(None);

        println!("{year} tests took {}us", sw.elapsed().as_micros());
    }
}

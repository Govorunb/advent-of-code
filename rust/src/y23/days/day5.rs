use std::ops::Range;
use crate::*;

aoc_day!(
    day = 5,
    output = usize,
    examples = [
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 35),
            (Self::INPUT, 806029445),
        ],
        test_cases![
            (Self::EXAMPLES[0], 46),
            // (Self::INPUT, 59370572), // bruteforce takes too long
        ]
    ],
    solve = |input, part| {
        let buh: Almanac = Almanac::parse(input, part);
        buh.brute_force()
    }
);


struct Almanac {
    seeds: Vec<Range<usize>>,
    maps: Vec<Map>,
}

struct Map {
    from: String,
    to: String,
    ranges: Vec<MapRange>,
}

#[derive(Debug, Clone)]
struct MapRange {
    dest: usize,
    source: usize,
    len: usize,
}

impl From<(usize, usize, usize)> for MapRange {
    fn from(input: (usize, usize, usize)) -> Self {
        let (dest, source, len) = input;
        Self { dest, source, len }
    }
}

impl From<&str> for MapRange {
    fn from(input: &str) -> Self {
        input
            .split_ascii_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect_tuple::<(_,_,_)>()
            .unwrap()
            .into()
    }
}

impl MapRange {
    fn dest_range(&self) -> Range<usize> {
        self.dest..(self.dest + self.len)
    }
    fn source_range(&self) -> Range<usize> {
        self.source..(self.source + self.len)
    }
    fn offset(&self) -> isize { self.dest as isize - self.source as isize }
}

impl Map {
    fn map_exact(&self, from: usize) -> usize {
        for range in self.ranges.as_slice() {
            if range.source_range().contains(&from) {
                return (from as isize + range.offset()) as usize;
            }
        }
        from
    }
    fn parse(input: &str) -> Self
    {
        let mut lines = input.lines();
        let (from, to) = lines.next().unwrap().split_once("-to-").unwrap();

        let ranges = lines
            .map_into()
            .collect_vec();
        Self {
            from: from.to_owned(),
            to: to.to_owned(),
            ranges,
        }
    }
}

impl Almanac {
    fn brute_force(&self) -> usize {
        self.seeds.par_iter()
            .cloned()
            .flatten()
            .map(|seed| self.maps.iter().fold(seed, |acc, m| m.map_exact(acc)))
            .min()
            .unwrap()
    }

    fn parse(input: &str, part: Part) -> Self {
        let mut sections = input.split("\n\n");

        let first_line = sections.next().unwrap()
            .split_once(':').unwrap()
            .1.trim();
        let seed_nums = first_line
            .split_ascii_whitespace()
            .map(|x| x.parse()
                .unwrap_or_else(|err| panic!("Invalid seed number: \"{x}\" {err:?}"))
            )
            .collect_vec();
        let seed_ranges = match part {
            Part::One => seed_nums.iter().map(|&x| x..x+1).collect_vec(),
            Part::Two => seed_nums.iter().tuples().map(|(&a, &b)| a..(a+b)).collect_vec(),
        };
        
        let maps = sections
            .map(Map::parse)
            .collect_vec();

        Self {
            seeds: seed_ranges,
            maps,
        }
    }
}

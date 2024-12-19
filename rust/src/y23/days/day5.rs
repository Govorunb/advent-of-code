#![allow(dead_code)]
use crate::*;
use std::ops::Range;

pub struct Day5;

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

fn overlaps(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.contains(&b.start) || a.contains(&b.end)
}

struct RangeSlice<T> {
    a_only: Vec<Range<T>>,
    both: Option<Range<T>>,
    b_only: Vec<Range<T>>,
}
fn slice(a: &Range<usize>, b: &Range<usize>) -> RangeSlice<usize> {
    if a.start > b.start {
        let mirrored = slice(b, a);
        return RangeSlice {
            a_only: mirrored.b_only,
            both: mirrored.both,
            b_only: mirrored.a_only,
        }
    }
    
    // a[   ]a b[  ]b
    // no overlap
    if a.end <= b.start {
        return RangeSlice {
            a_only: vec![a.clone()],
            both: None,
            b_only: vec![b.clone()],
        };
    }
    
    let mut left: Vec<Range<usize>> = Vec::new();
    let mut right: Vec<Range<usize>> = Vec::new();
    
    let both: Option<Range<usize>> = if b.start == a.end {
        None
    } else {
        Some(b.start..a.end)
    };
    
    if a.start != b.start {
        left.push(a.start..b.start);
    }
    if a.end != b.end {
        // a[   b[  ]b  ]a
        if a.end > b.end {
            left.push(b.end..a.end);
        // a[   b[  ]a   ]b
        } else {
            right.push(a.end..b.end);
        }
    }

    if left.iter().any(|r| r.is_empty()) {
        panic!("uhhhhhh")
    }

    if right.iter().any(|r| r.is_empty()) {
        panic!("uhhhhhhhhhhhhhhhhh")
    }
    if let Some(adsasd) = both.clone() {
        if adsasd.is_empty() {
            panic!("bruhhhh")
        }
    }

    RangeSlice {
        a_only: left,
        both,
        b_only: right
    }
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

    fn map_range(&self, map_1to2: &MapRange, category_2to3: &mut Vec<MapRange>) -> Vec<MapRange> {
        let mut maps_1to3 = Vec::new();
        let mut _map_1to2 = map_1to2.clone();
        let mut overlapped_any = false;
        let mut i2to3: usize = 0;
        while i2to3 < category_2to3.len() {
            let range2_2to3 = category_2to3[i2to3].source_range();
            let range2_1to2 = map_1to2.dest_range();
            let slice = slice(&range2_1to2, &range2_2to3);

            if let Some(overlapping) = slice.both {
                overlapped_any = true;
                let mapped_both = self.map_both(&_map_1to2, &category_2to3[i2to3], overlapping);
                maps_1to3.push(mapped_both);
            }
            match slice.a_only.len() {
                0 => {

                },
                1 => {
                    let outer_l = slice.a_only[0].clone();
                    let len = outer_l.len();
                    let amt_cut = map_1to2.len - len;
                    if outer_l.start <= map_1to2.dest {
                        // cut right side
                        _map_1to2.len = len;
                    } else {
                        // cut left side
                        _map_1to2.source += amt_cut;
                        _map_1to2.dest += amt_cut;
                        _map_1to2.len = len;
                    }                    
                }
                _ => {
                    for outer_l in slice.a_only {
                        let len = outer_l.len();
                        let amt_cut = map_1to2.len - len;
                        let mut to_insert_back = map_1to2.clone();
                        if outer_l.start <= map_1to2.dest {
                            // cut right side
                            to_insert_back.len = len;
                        } else {
                            // cut left side
                            to_insert_back.source += amt_cut;
                            to_insert_back.dest += amt_cut;
                            to_insert_back.len = len;
                        }
                        let sub = self.map_range(&to_insert_back, category_2to3);
        
                        maps_1to3.extend(sub);
                    }
                }
            }
            match slice.b_only.len() {
                0 => {
                    i2to3 += 1;
                },
                1 => {
                    let curr_r = &mut category_2to3[i2to3];
                    let outer_r = slice.b_only[0].clone();
                    let len = outer_r.len();
                    if curr_r.len < len
                    {
                        println!("break here");
                    }
                    let amt_cut = curr_r.len - len;
                    if outer_r.start <= curr_r.source {
                        // cut right side
                        curr_r.len = len;
                    } else {
                        // cut left side
                        curr_r.source += amt_cut;
                        curr_r.dest += amt_cut;
                        curr_r.len = len;
                    }
                    i2to3 += 1;
                },
                _ => {
                    let curr_r = category_2to3[i2to3].clone();
                    category_2to3.swap_remove(i2to3);
                    
                    for outer_r in slice.b_only {
                        let len = outer_r.len();
                        if curr_r.len < len
                        {
                            println!("break here");
                        }
                        let amt_cut = curr_r.len - len;
                        let mut to_insert_back = curr_r.clone();
                        if outer_r.start <= curr_r.source {
                            // cut right side
                            to_insert_back.len = len;
                        } else {
                            // cut left side
                            to_insert_back.source += amt_cut;
                            to_insert_back.dest += amt_cut;
                            to_insert_back.len = len;
                        }
                        category_2to3.push(to_insert_back);
                    }
                }
            }
        }

        if !overlapped_any {
            maps_1to3.push(map_1to2.clone());
        }
        
        // leftovers from 2 are added outside bc they can overlap multiple 1s
        maps_1to3
    }

    fn map_both(&self, map_1to2: &MapRange, map_2to3: &MapRange, range2: Range<usize>) -> MapRange {
        if range2.start < map_1to2.dest
        {
            println!("break here 2");
        }
        if range2.start < map_2to3.source
        {
            println!("break here 3");
        }
        let off1 = range2.start - map_1to2.dest;
        let off2 = range2.start - map_2to3.source;
        MapRange {
            dest: map_2to3.dest + off2,
            source: map_1to2.source + off1,
            len: range2.len(),
        }
    }

    fn map_solo(&self, map: &MapRange, range2: Range<usize>) -> MapRange {
        let start_2_offset = range2.start - map.dest;
        MapRange {
            dest: map.dest + start_2_offset,
            source: map.source + start_2_offset,
            len: range2.len(),
        }
    }

    fn flatten_layer(&self, category_1to2: &[MapRange], category_2to3: &[MapRange]) -> Vec<MapRange> {
        let mut ranges: Vec<MapRange> = Vec::new();
        let (left_overlaps, left_solo): (Vec<_>, Vec<_>) = category_1to2.iter()
            .partition(|&l| {
                category_2to3.iter().any(|r| overlaps(&l.dest_range(), &r.source_range()))
            });
        let right_solo = category_2to3.iter().filter(|&r| {
            !category_1to2.iter().any(|l| overlaps(&l.dest_range(), &r.source_range()))
        }).collect_vec();
        ranges.extend(left_solo.iter().map(|l| self.map_solo(l, l.dest_range())));
        ranges.extend(right_solo.iter().map(|r| self.map_solo(r, r.dest_range())));
        
        let overlapping_sets = left_overlaps.iter()
            .map(|&l| {
                (
                    l,
                    category_2to3.iter()
                        .filter(|r| overlaps(&l.dest_range(), &r.source_range()))
                        .cloned()
                        .collect_vec(),
                )
            }).collect_vec();

        for (l, mut r) in overlapping_sets {
            let mapped_l = self.map_range(l, &mut r);
            ranges.extend(mapped_l.into_iter().filter(|x| x.len > 0));
            // map_range slices off overlaps in r
            // so now only "r_only" segments should be left
            // this is technically incorrect as one R can overlap many Ls
            // but i honestly don't care
            ranges.extend(r.iter()
                .map(|r| self.map_solo(r, r.dest_range()))
                .filter(|x| x.len > 0)
            );
        }

        ranges
    }

    fn parse(input: &str, part: Part) -> Self {
        let mut sections = input.split("\n\n");

        let first_line = sections.next().unwrap()
            .split_once(':').unwrap()
            .1.trim();
        let seed_nums = first_line
            .split(' ')
            .map(|x|
                x.trim()
                .parse()
                .unwrap_or_else(|err| panic!("Invalid seed number: \"{x}\" {err:?}"))
            )
            .collect_vec();
        let seed_ranges = match part {
            Part::One => seed_nums.iter().map(|&x| x..x+1).collect_vec(),
            Part::Two => seed_nums.chunks(2).map(|x| x[0]..(x[0]+x[1])).collect_vec(),
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

impl Day<5> for Day5 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day5.txt");

    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let buh: Almanac = Almanac::parse(input, part);
        buh.brute_force()
    }
    const EXAMPLES: &'static [&'static str] = &[
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
56 93 4
"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 35),
                (Self::INPUT, 806029445),
            ],
            test_cases![
                (Self::EXAMPLES[0], 46),
                // (Self::INPUT, 59370572), // bruteforce takes too long
            ]
        ]
    }
}

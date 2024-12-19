// replace all 12 with the day number
#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use crate::*;

pub struct Day12;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Row {
    original: String,
    curr: String,
    sizes: Vec<usize>,
}

const WORKING: char = '.';
const BROKEN: char = '#';
const UNKNOWN: char = '?';

impl Row {
    fn parse(input: &str) -> Self {
        let (chars_str, sizes_str) = input.split_once(' ').unwrap();
        let sizes = sizes_str
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();
        let mut chars: Vec<char> = Vec::new();
        
        let mut last = '\0';
        // replace "..." with "." (they mean the same thing)
        for c in chars_str.chars() {
            if last == WORKING && c == WORKING {
                continue;
            }
            last = c;
            chars.push(c);
        }

        Self {
            original: input.to_owned(),
            curr: chars.iter().collect::<String>(),
            sizes,
        }
    }

    fn all_str(&self) -> String {
        self.curr.clone()
    }
    fn sizes_str(&self) -> String {
        self.sizes.iter().join(",")
    }

    fn reduce(&mut self) {
        let mut last: String = String::new();
        while last != self.curr {
            last = self.curr.clone();
            if last.is_empty() {
                break;
            }
            // . at start/end are useless
            self.trim_working();
            // full # runs are also useless
            self.trim_known_broken();
            self.trim_working();
            // partial # runs (like (?##, 3) or (#?#?, 4)) - we can fill and remove these in some cases
            self.trim_trivial_unknowns();
        }
    }
    fn trim_working(&mut self) {
        self.curr = self.curr.trim_matches(WORKING).to_string();
    }

    fn trim_known_broken(&mut self) {
        if !self.sizes.is_empty() {
            let start_run = self.curr.chars()
                .take_while(|&c| c == BROKEN)
                .collect_vec();
            if &start_run.len() == self.sizes.first().unwrap() {
                self.sizes.remove(0);
                self.remove_front_known(start_run.len());
            }
        }
        if !self.sizes.is_empty() {
            let end_run = self.curr.chars()
                .rev()
                .take_while(|&c| c == BROKEN)
                .collect_vec();
            if &end_run.len() == self.sizes.last().unwrap() {
                self.sizes.pop();
                self.remove_tail_known(end_run.len());
            }
        }
    }

    fn remove_front_chars(&mut self, count: usize) {
        self.curr.drain(0..count);
    }

    fn remove_tail_chars(&mut self, count: usize) {
        self.curr.truncate(self.curr.len() - count);
    }

    fn remove_front_known(&mut self, count: usize) {
        let mut to_remove = count;
        if self.curr.len() > to_remove {
            to_remove += 1; // include separator
        }
        self.remove_front_chars(to_remove);
    }

    fn remove_tail_known(&mut self, count: usize) {
        let mut to_remove = count;
        if self.curr.len() > to_remove {
            to_remove += 1; // include separator
        }
        self.remove_tail_chars(to_remove);
    }

    fn trim_trivial_unknowns(&mut self) {
        // takes care of cases like "###? 4" and "?### 4"
        // these are only trivial when the unknowns are only on one side
        // (or when your remaining row is X unknowns and needs one X-sized group)
        // (or also when your starting run is X and you need >X)
        // so e.g. for "?###? 4" there isn't "only one way" to fill it in
        // but "###????? 4,2" happily reduces into "####.?? 2" (and then to "" in a later iteration)
        // reducing from end is the same as from start (just with the string reversed)
        // additionally, it can reduce "??.???? 3" (reduces to "???? 3")
        
        let nothing_to_match = self.sizes.is_empty();
        let all_matched_minimally = self.curr.len() == self.min_size_to_fit();
        if nothing_to_match || all_matched_minimally
        {
            // nothing else to reduce
            self.curr.clear();
            return;
        }
        
        if self.sizes.is_empty() {return;}
        match self.curr.find(WORKING) {
            None => {return;},
            Some(idx) => {
                let from_start = &self.curr.clone()[..idx];
                self.trim_partial_broken_(from_start, true);
            }
        }
        if self.sizes.is_empty() {return;}
        match self.curr.rfind(WORKING) {
            None => {},
            Some(idx) => {
                let from_end = &self.curr.clone()[idx..];
                self.trim_partial_broken_(from_end, false);
            }
        }
    }
    
    fn trim_partial_broken_(&mut self, taken: &str, is_front: bool)
    {
        if taken.is_empty() {
            return;
        }
        let broken_len = if is_front { self.sizes[0] } else { *self.sizes.last().unwrap() };
        if taken.len() < broken_len && taken.chars().all(|c| c == UNKNOWN) {
            //println!("remove {} from {} - all unknowns (not enough to fill - have {} need {})", broken_len, if is_front {"start"} else {"end"}, taken.len(), broken_len);
            if is_front {
                self.sizes.remove(0);
                self.remove_front_chars(taken.len());
            } else {
                self.sizes.pop();
                self.remove_tail_chars(taken.len());
            }
            return;
        }
        let known_start = taken.starts_with('#');
        let known_end = taken.len() == broken_len
            && taken.chars().nth(broken_len-1) == Some('#');
        
        if taken.len() == broken_len || known_start || known_end
        {
            // println!("remove {} from {} - trivial broken/unknown '{}'", broken_len, if is_front {"start"} else {"end"}, &taken[..broken_len]);
            if is_front {
                self.sizes.remove(0);
                self.remove_front_known(broken_len);
            } else {
                self.sizes.pop();
                self.remove_tail_known(broken_len);
            }
        }
    }

    fn ways(&self) -> usize {
        if self.curr.is_empty() || self.sizes.is_empty() // fully reduced, all trivial -> only one way
        || self.curr.len() == self.min_size_to_fit() { // not enough space to fill unknowns more than one way
            1
        }
        else if self.curr.find(UNKNOWN).is_none() {
            panic!("no unknowns, idk if 0 or 1");
        }
        else {
            todo!("TODO {} {} (original {})", self.all_str(), self.sizes_str(), self.original);
            
        }
    }

    fn min_size_to_fit(&self) -> usize {
        Self::min_space_to_fit(&self.sizes)
    }
    fn min_space_to_fit(sizes: &[usize]) -> usize {
        if sizes.is_empty() {
            0
        } else {
            sizes.iter().sum::<usize>() + sizes.len() - 1
        }
    }
    fn run_ways(state: char, len: usize, sizes: &[usize]) -> usize {
        if state != UNKNOWN {
            panic!("should not be calling this on known");
        }
        let need = Row::min_space_to_fit(sizes);
        if len < need {
            return 0;
        }
        if sizes.len() == 1 {
            return Self::combinations(len, need);
        }
        let head = &sizes[..1];
        let tail = &head[1..];
        let need_for_head = Row::min_space_to_fit(head);
        let need_for_tail = Row::min_space_to_fit(tail);
        let have_space_for_tail = len - (need_for_head + 1); // +1 for the separator
        let have_space_for_head = len - (need_for_tail + 1); // ditto
        let head_combinations = Self::run_ways(state, have_space_for_head, head);
        let tail_combinations = Self::run_ways(state, have_space_for_tail, tail);
        
        head_combinations + tail_combinations
    }

    fn combinations(have: usize, need: usize) -> usize {
        if have < need {0} else {1 + have - need}
    }
}

impl Day<12> for Day12 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day12.txt");

    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let mut record = input.lines()
            .map(Row::parse)
            .collect_vec();
        for row in record.as_mut_slice() {
            row.reduce();
        }
        match part {
            Part::One => {
                record.iter()
                    .map(|row| row.ways())
                    .sum()
            },
            Part::Two => {
                0
            }
        }
    }
    const EXAMPLES: &'static [&'static str] = &[
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                // goes from least to most ambiguous
                ("? 1", 1),
                ("??? 3", 1),
                
                ("###? 3", 1),
                ("###?? 3", 1),
                ("?#? 3", 1),
                ("??? 1,1", 1),
                ("??#?#### 3,4", 1),
                ("###...?..?... 3,1,1", 1),
                ("???...?..??...? 3", 1),
                ("????? 3,1", 1),
                ("?????. 3,1", 1),
                ("???..?##?.?. 3,2,1", 1),
                
                ("?? 1", 2),
                ("?#?? 3", 2),
                ("??#? 3", 2),
                ("??#?? 3", 3),
                ("??#?####...?? 3,4,1", 2),
                ("??#??#### 3,4", 2),
                ("###...?..?... 3,1", 2),
                ("?????? 3,1", 3),
                
                ("??.?? 1,1", 4),
                ("???.?? 1,1", 7),
                (Self::EXAMPLES[0], 21),
                //(Self::INPUT, 0),
            ],
            test_cases![
                //(Self::EXAMPLES[0], 0),
                //(Self::INPUT, 0),
            ]
        ]
    }
}


impl Day12 {
    pub fn test_reduce(&self)
    {
        let inputs = vec![
            ("...## 2", ""),
            (".#.#.# 1,1,1", ""),
            ("?? 1", "??"),
            ("??.....?? 1,1", "??.??"),
            ("??? 3", ""),
            ("###? 3", ""),
            ("###?? 3", ""),
            ("?#? 3", ""),
            ("?#?? 3", "?#??"),
            ("??#? 3", "??#?"),
            ("??#?? 3", "??#??"),
            ("??#?#### 3,4", ""),
            ("??#??#### 3,4", "??#?"),
            ("??...???..??. 3,1", "???.??"), // in an ideal world this would be "??"
            ("??...???.???.?? 3,1", "???.???.??"),
        ];
        for (i, (input, expected)) in inputs.iter().cloned().enumerate() {
            let mut row = Row::parse(input);
            row.reduce();
            let got = row.curr;
            assert_eq!(expected, got, "reduce case {} - expected {expected}, got {got}", i+1);
        }
    }
}
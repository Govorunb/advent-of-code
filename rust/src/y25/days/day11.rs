use pathfinding::prelude::{astar_bag, astar_bag_collect, count_paths};

use std::{collections::VecDeque, hash::Hash, iter};

use crate::*;

aoc_day!(
    day = 11,
    output = usize,
    examples = [
"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",

"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 5),
            (Self::INPUT, 534),
        ],
        test_cases![
            (Self::EXAMPLES[1], 2),
            (Self::INPUT, 499645520864100),
        ]
    ],
    solve = |input, part| {
        let lines = input.lines();
        let connections: FxHashMap<_, _> = lines.map(|l| {
            let (from, to) = l.split_once(": ").unwrap();
            let to = to.split_ascii_whitespace().collect_vec();
            (from, to)
        }).collect();
        let succ = |s: &&str| connections.get(s).cloned().unwrap_or_default();
        match part {
            Part::One => {
                count_paths("you", succ, |&s| s == "out")
            },
            Part::Two => {
                let must_visit = ["dac", "fft"]; // any order :)

                // return must_visit.into_iter()
                //     .permutations(must_visit.len())
                //     .map(|perm| iter::once("svr").chain(perm).chain(iter::once("out"))
                //         .tuple_windows()
                //         .map(|(a, b)| count_paths(a, succ, |&s| s == b))
                //         .product::<usize>()
                //     ).sum::<usize>();

                // expands to:
                //   svr->fft * fft->dac * dac->out
                // + svr->dac * dac->fft * fft->out
                // (only one will be non-zero since the graph is acyclic - can't have fft->dac and dac->fft)

                // ever so slightly faster due to breaking earlier
                for perm in must_visit.into_iter().permutations(must_visit.len()) {
                    let mut count = 1;
                    for (a,b) in iter::once("svr").chain(perm).chain(iter::once("out")).tuple_windows() {
                        count *= count_paths(a, succ, |&s| s == b);
                        if count == 0 {break};
                    }
                    if count != 0 {return count};
                }
                unreachable!()
            }
        }
    }
);

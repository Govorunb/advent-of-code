use crate::*;

aoc_day!(
    day = 5,
    output = usize,
    examples = [
"3-5
10-14
16-20
12-18

1
5
8
11
17
32"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 3),
            (Self::INPUT, 840),
        ],
        test_cases![
            (Self::EXAMPLES[0], 14),
            (Self::INPUT, 359913027576322),
        ]
    ],
    solve = |input, part| {
        let (ranges, available) = input.split_once("\n\n").unwrap();
        let ranges = ranges.lines().map(|l| {
            let (s, e) = l.split_once('-').unwrap();
            (s.parse::<usize>().unwrap())..=(e.parse::<usize>().unwrap())
        }).collect_vec();
        match part {
            Part::One => {
                let available = available.lines().map(|l| l.parse::<usize>().unwrap()).collect_vec();
                available.iter()
                    .filter(|i| ranges.iter().any(|r| r.contains(&i)))
                    .count()
            },
            Part::Two => {
                let mut merge = vec![];
                let mut set: FxHashSet<std::ops::RangeInclusive<usize>> = FxHashSet::from_iter(ranges);
                loop {
                    // funny borrow checker
                    for (r1, r2) in set.iter().cloned().cartesian_product(set.iter().cloned()) {
                        if r1 == r2 {continue}
                        if r1.contains(r2.start()) | r1.contains(r2.end()) {
                            merge.push((r1, r2));
                        }
                    }
                    if merge.is_empty() {break}
                    
                    for (r1, r2) in merge.drain(..) {
                        set.remove(&r1);
                        set.remove(&r2);
                        
                        let (r1, r2) = if r1.start() <= r2.start() {
                            (r1, r2)
                        } else {
                            (r2, r1)
                        };
                        
                        // TODO: write a thing for merging ranges already
                        if r1.start() <= r2.start() {
                            // 1.....2______
                            if r1.end() >= r2.end() {
                                // r1 contains r2
                                // 1..2__2..1
                                set.insert(r1);
                            } else if r1.end() >= r2.start() {
                                //  1..2_1__2
                                set.insert(*r1.start() ..= *r2.end());
                            } else {
                                panic!("huh? {r1:?} - {r2:?}");
                                // set.insert(r1);
                                // set.insert(r2);
                            }
                        } else {
                            unreachable!();
                        }
                    }
                }
                // now all ranges are non-overlapping
                set.into_iter()
                    .map(|r| r.size_hint().0)
                    .sum()
            }
        }
    }
);

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
        let mut ranges = ranges.lines().map(|l| {
            let (s, e) = l.split_once('-').unwrap()
                .map(|s| s.parse::<usize>().unwrap());
            s..=e
        }).collect_vec();
        ranges.sort_by_key(|r| *r.start());
        match part {
            Part::One => {
                available.lines()
                    .map(|l| l.parse::<usize>().unwrap())
                    .filter(|i| {
                        ranges.iter()
                            .take_while(|r| r.start() <= i)
                            .any(|r| r.end() >= i)
                    })
                    .count()
            },
            Part::Two => {
                // because the ranges are sorted, (r1.start<=r2.start) always holds true
                // if we can get rid of the set (we can), merging can be done much faster
                let mut merge = vec![];
                let mut set = FxHashSet::from_iter(ranges);
                loop {
                    for (r1, r2) in set.iter().sorted_by_key(|r| *r.start()).triangle_product() {
                        if r1 == r2 {continue}
                        
                        if r1.end() >= r2.start() {
                            merge.push((r1.clone(), r2.clone()));
                        }
                    }
                    if merge.is_empty() {break}
                    
                    for (r1, r2) in merge.drain(..) {
                        set.remove(&r1);
                        set.remove(&r2);
                        
                        set.insert(merge_ranges(r1, r2).unwrap());
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

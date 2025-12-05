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
        
        // putting p2 before p1 to make p1 faster teehee
        let mut merged: Vec<std::ops::RangeInclusive<usize>> = vec![];
        for r in ranges {
            if let Some(prev) = merged.last_mut() {
                // because ranges are sorted, prev.start <= r.start already always holds
                // now we immediately know whether to merge (and can also immediately merge)
                if prev.end() >= r.start() {
                    *prev = merge_ranges(prev, &r).unwrap();
                    continue;
                }
            }
            merged.push(r);
        }
        // now all ranges are non-overlapping

        match part {
            Part::One => {
                // if both are sorted, we're able to walk through both collections only once* (excluding the walk for sorting)
                let ids = available.lines()
                    .map(|l| l.parse::<usize>().unwrap())
                    .sorted()
                    .collect_vec(); // free to collect after sorting
                let mut count = 0;
                let mut i = 0;
                for range in &merged {
                    while ids[i] < *range.start() {
                        i += 1;
                        if i >= ids.len() {return count};
                    }
                    while ids[i] <= *range.end() {
                        count += 1;
                        i += 1;
                        if i >= ids.len() {return count};
                    }
                }
                count
            },
            Part::Two => {
                merged.into_iter()
                    .map(|r| r.size_hint().0)
                    .sum()
            }
        }
    }
);

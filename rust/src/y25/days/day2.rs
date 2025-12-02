use crate::*;

aoc_day!(
    day = 2,
    output = usize,
    examples = [
"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 1227775554),
            (Self::INPUT, 22062284697),
        ],
        test_cases![
            (Self::EXAMPLES[0], 4174379265),
            (Self::INPUT, 46666175279),
        ]
    ],
    solve = |input, part| {
        match part {
            Part::One => {
                let log_table = (0..=16).map(|i| 10usize.pow(i)).collect_vec();
                input.par_split(',').map(|pair| {
                    let (first, second) = pair.trim().split_once('-').unwrap();
                    let [start, end] = [first, second].map(|s| s.parse::<usize>().expect(s));
                    (start..=end).into_par_iter()
                        .filter_map(|i| {
                            let digits = (i as f64).log10().ceil() as usize;
                            let half_point = log_table[digits/2];
                            let bottom_half = i % half_point;
                            let top_half = i / half_point;
                            // dbg!(i, digits, half_point, top_half, bottom_half);
                            if top_half == bottom_half {
                                return Some(i);
                            }
                            None
                        }).sum::<usize>()
                }).sum()
            },
            Part::Two => {
                input.par_split(',').map(|pair| {
                    let (first, second) = pair.trim().split_once('-').unwrap();
                    let [start, end] = [first, second].map(|s| s.parse::<usize>().expect(s));
                    (start..=end).into_par_iter()
                        .filter_map(|i| {
                            let s = i.to_string();
                            for j in 1..=(s.len()/2) {
                                if s.len() % j != 0  { continue }
                                
                                let parts = s.bytes().chunks(j);
                                // dbg!(s.len() / j);
                                if parts.into_iter().map(|c| c.collect_vec()).all_equal() {
                                    // println!("invalid id {i}");
                                    return Some(i);
                                }
                            }
                            None
                        }).sum::<usize>()
                }).sum()
            }
        }
    }
);

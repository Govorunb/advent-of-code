use num::Float;

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
            ("11-22", [11,22].iter().sum()),
            ("95-115", [99,111].iter().sum()),
            ("998-1012", [999,1010].iter().sum()),
            ("1188511880-1188511890", 1188511885),
            ("222220-222224", 222222),
            ("1698522-1698528", 0),
            ("446443-446449", 446446),
            ("38593856-38593862", 38593859),
            ("565653-565659", 565656),
            ("824824821-824824827", 824824824),
            ("2121212118-2121212124", 2121212121),
            (Self::EXAMPLES[0], 4174379265),
            (Self::INPUT, 46666175279),
        ]
    ],
    solve = |input, part| {
        let pow10_table = (0..=16).map(|i| 10usize.pow(i)).collect_vec();
        let ids = input.par_split(',')
            .map(|pair| {
                let (first, second) = pair.trim().split_once('-').unwrap();
                let [start, end] = [first, second].map(|s| s.parse::<usize>().expect(s));
                (start, end)
            });
        match part {
            Part::One => {
                ids.map(|(start, end)| {
                    // small trick - all numbers between start/end MAY have the same digit count
                    let digits_start = digits(start);
                    let digits_end = digits(end);
                    let digits_maybe = (digits_start == digits_end).then_some(digits_start);
                    (start..=end).into_par_iter()
                        .map(|i| {
                            // ex: i = 1234567890
                            // digits = 10; half digits = 5 -> half_point = 10^5 = 100000
                            // 12345..... / 1_00000
                            // .....67890 % 1_00000
                            let i_digits = digits_maybe.unwrap_or_else(|| digits(i));
                            let half_point = pow10_table[i_digits/2];
                            let top_half = i / half_point;
                            let bottom_half = i % half_point;
                            // dbg!(i, digits, half_point, top_half, bottom_half);
                            if top_half == bottom_half {
                                return i;
                            }
                            0
                        }).sum::<usize>()
                }).sum()
            },
            Part::Two => {
                ids.map(|(start, end)| {
                    let digits_start = digits(start);
                    let digits_end = digits(end);
                    let digits_maybe = (digits_start == digits_end).then_some(digits_start);
                    (start..=end).into_par_iter()
                        .map(|i| {
                            let i_digits = digits_maybe.unwrap_or_else(|| digits(i));
                            'outer: for div in (2..=i_digits).filter(|d| i_digits % d == 0) {
                                let cut = pow10_table[i_digits / div];
                                let mut curr = i;
                                let mut comp = curr % cut;
                                while curr > 0 {
                                    let comp2 = curr % cut;
                                    if comp != comp2 {
                                        continue 'outer;
                                    } else {
                                        comp = comp2;
                                    };
                                    curr /= cut;
                                }
                                return i;
                            }
                            0
                            // let s = i.to_string();
                            // for j in 1..=(s.len()/2) {
                            //     if s.len() % j != 0  { continue }

                            //     let parts = s.bytes().chunks(j);
                            //     // dbg!(s.len() / j);
                            //     if parts.into_iter().map(|c| c.collect_vec()).all_equal() {
                            //         // println!("invalid id {i}");
                            //         return i;
                            //     }
                            // }
                            // 0
                        }).sum::<usize>()
                }).sum()
            }
        }
    }
);

fn digits(num: usize) -> usize {
    (num as f64).log10().ceil() as usize
}
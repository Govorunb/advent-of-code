// replace all 19 with the day number
use crate::*;

pub struct Day19 {
    
}

fn p1(avail: &[&str], target: &str) -> bool {
    let res = p1_funcy(avail, target);
    debug_assert_eq!(res, p1_impy(avail, target));
    res
}

fn p1_funcy(avail: &[&str], target: &str) -> bool {
    avail.iter()
        .any(|&pat| target.strip_prefix(pat).is_some_and(|next| next.is_empty() || p1_funcy(avail, next)))
}

fn p1_impy(avail: &[&str], target: &str) -> bool {
    for &pat in avail {
        let Some(next) = target.strip_prefix(pat)
            else {continue};
        if next.is_empty() || p1_impy(avail, next) {
            return true;
        }
    }
    false
}
fn p2(avail: &[&str], target: &str) -> usize {
    let res = p2_funcy(avail, target, &mut FxHashMap::default());
    debug_assert_eq!(res, p2_impy(avail, target, &mut FxHashMap::default()));
    res
}

fn p2_funcy<'a>(avail: &[&str], target: &'a str, dp: &mut FxHashMap<&'a str, usize>) -> usize {
    dp.get(target).cloned().unwrap_or_else(||
        avail.iter()
            .filter_map(|pat| target.strip_prefix(pat))
            .map(|next| {
                if next.is_empty() { return 1 };

                let res = p2_funcy(avail, next, dp);
                dp.insert(next, res);
                res
            }).sum()
    )
}

fn p2_impy<'a>(avail: &[&str], target: &'a str, dp: &mut FxHashMap<&'a str, usize>) -> usize {
    if let Some(&seen) = dp.get(target) {return seen};
    
    let mut out = 0;
    for &pat in avail {
        let Some(next) = target.strip_prefix(pat)
            else {continue};
        if next.is_empty() {
            out += 1;
        } else {
            let res = p2_impy(avail, next, dp);
            out += res;
            dp.insert(next, res);
        };
    }
    out
}

impl Day<19> for Day19 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day19.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let (avail_s, pats_s) = input.split_once("\n\n").unwrap();
        let avail = avail_s.split(", ").collect_vec();
        match part {
            Part::One => {
                // let regex = Regex::new(&format!(r#"^(?:{})+$"#, avail.join("|"))).unwrap();
                pats_s.lines()
                    // .filter(|l| regex.is_match(l))
                    .filter(|l| p1(&avail, l))
                    .count()
            },
            Part::Two => {
                // grrrrr i can't regex anymore
                pats_s.lines()
                    .map(|line| p2(&avail, line))
                    .sum()
            }
        }
    }
    const EXAMPLES: &'static [&'static str] = &[
"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 6),
                (Self::INPUT, 233),
            ],
            test_cases![
                (Self::EXAMPLES[0], 16),
                (Self::INPUT, 691316989225259),
            ]
        ]
    }
}

impl Default for Day19 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day19 {
    pub fn new() -> Self {
        Self {
        }
    }
}
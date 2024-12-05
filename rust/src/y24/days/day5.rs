// replace all 5 with the day number
use crate::*;

pub const DAY5_EXAMPLE: &str =
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

pub struct Day5 {
    
}

struct Rule {
    print: usize,
    before: usize,
}

impl Day<5> for Day5 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day5.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let bruh = input.replace("\r", "");
        let (rules_s, updates_s) = bruh.split_once("\n\n").unwrap();
        let rules = rules_s.lines().map(|rs| {
            let (x,y) = rs.split_once('|').unwrap();
            Rule { print: x.parse().unwrap(), before: y.parse().unwrap()}
        }).collect_vec();
        let mut rules_map: FxHashMap<usize, Vec<usize>> = Default::default();
        for rule in rules {
            rules_map.entry(rule.print).or_insert(vec![]);
            rules_map.get_mut(&rule.print).unwrap().push(rule.before);
        }
        let mut updates = updates_s.lines().map(|us| {
            us.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec()
        }).collect_vec();
        match part {
            Part::One => {
                updates.iter().filter(|&u| {
                    if u.len() < 2 {return false}
                    
                    for i in 1..u.len() {
                        let item = u[i];
                        if let Some(should_be_before) = rules_map.get(&item) {
                            if let Some(bad) = u.iter().take(i).find(|c| should_be_before.contains(c)) {
                                // println!("{} found before {}", bad, item);
                                return false
                            }
                        }
                    }
                    true
                })
                    .map(|u| {
                        // println!("{:?}", u);
                        u[u.len() / 2] // middle
                    }).sum()
            },
            Part::Two => {
                // reorder incorrect updates
                updates.iter_mut().filter_map(|u| {
                    if u.len() < 2 {panic!()}

                    let mut reordered = false;
                    let mut done = false;
                    while !done {
                        done = true;
                        for i in 1..u.len() {
                            let item = u[i];
                            if let Some(should_be_before) = rules_map.get(&item) {
                                let mut bad: Option<usize> = None;
                                // mut/unmut shenanigans
                                for j in 0..i {
                                    if should_be_before.contains(&u[j]) {
                                        bad = Some(j);
                                    }
                                }
                                if let Some(badj) = bad {
                                    // println!("{} found before {}", u[badj], item);
                                    reordered = true;
                                    done = false;
                                    u.swap(i, badj);
                                }
                            }
                        }
                    }
                    if reordered { Some(u) } else {None}
                }).map(|u| {
                    // println!("{:?}", u);
                    u[u.len() / 2] // middle
                }).sum()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY5_EXAMPLE, 143),
                (self.input(), 4924),
            ],
            test_cases![
                (DAY5_EXAMPLE, 123),
                // (self.input(), 0),
            ]
        ]
    }
}

impl Default for Day5 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day5 {
    pub fn new() -> Self {
        Self {
        }
    }
}
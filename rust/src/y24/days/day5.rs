use crate::*;

aoc_day!(
    day = 5,
    output = usize,
    examples = [
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
97,13,75,29,47"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 143),
            (Self::INPUT, 4924),
        ],
        test_cases![
            (Self::EXAMPLES[0], 123),
            // (Self::INPUT, 0),
        ]
    ],
    solve = |input, part| {
        let (rules_s, updates_s) = input.split_once("\n\n").unwrap();
        let rules = rules_s.lines().map(|rs| {
            let (x,y) = rs.split_once('|').unwrap();
            Rule { print: x.parse().unwrap(), before: y.parse().unwrap()}
        }).collect_vec();
        let mut rules_map: FxHashMap<usize, Vec<usize>> = Default::default();
        for rule in rules {
            rules_map.entry(rule.print).or_default();
            rules_map.get_mut(&rule.print).unwrap().push(rule.before);
        }
        let mut updates = updates_s.lines().map(|us| {
            us.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec()
        }).collect_vec();
        match part {
            Part::One => {
                updates.iter()
                    .filter(|&u| {
                        if u.len() < 2 {return false}
                        
                        for (i, item) in u.iter().enumerate() {
                            if let Some(should_be_before) = rules_map.get(item) {
                                if let Some(_bad) = u.iter().take(i).find(|c| should_be_before.contains(c)) {
                                    // println!("{} found before {}", _bad, item);
                                    return false
                                }
                            }
                        }
                        true
                    }).map(|u| {
                        // println!("{:?}", u);
                        u[u.len() / 2] // middle
                    }).sum()
            },
            Part::Two => {
                // reorder incorrect updates
                updates.iter_mut().filter_map(|u| { // .filter has weird inference issues, icba
                    debug_assert!(u.len() > 1);

                    let mut reordered = false;
                    let mut done = false;
                    while !done {
                        done = true;
                        // borrow checker prevents u.iter().enumerate()
                        for i in 1..u.len() {
                            let item = u[i];
                            if let Some(should_be_before) = rules_map.get(&item) {
                                let option = u.iter().enumerate()
                                    .take(i)
                                    .find_map(|(j, c)|
                                        should_be_before.contains(c)
                                            .then_some(j)
                                    );
                                if let Some(badj) = option {
                                    // println!("{} found before {}", u[badj], item);
                                    reordered = true;
                                    done = false;
                                    u.swap(i, badj);
                                }
                            }
                        }
                    }
                    if reordered {Some(u)} else {None}
                }).map(|u| {
                    // println!("{:?}", u);
                    u[u.len() / 2] // middle
                }).sum()
            }
        }
    }
);

struct Rule {
    print: usize,
    before: usize,
}

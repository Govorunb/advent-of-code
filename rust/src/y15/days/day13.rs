use std::collections::HashSet;
use rustc_hash::FxBuildHasher;
use crate::*;

pub const DAY13_EXAMPLE: &str =
"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

pub struct Day13 {
    
}

impl Day<13> for Day13 {
    type Output = isize;
    const INPUT: &'static str = include_str!("../Input/day13.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        // graph search again... guh
        let regex = Regex::new(r#"(?<name>\w+) would (?<sign>lose|gain) (?<amt>\d+) happiness units by sitting next to (?<other>\w+)."#).unwrap();
        let mut names: FxHashSet<String> = Default::default();
        let mut rules: FxHashMap<(String, String), isize> = regex.captures_iter(input)
            .map(|c| {
                let edge = (c.string("name"), c.string("other"));
                names.insert(edge.0.clone());
                let mut amt = c.isize("amt");
                if "lose" == c.str("sign") {
                    amt = -amt;
                }
                (edge, amt)
            })
            .collect();
        if let Part::Two = part {
            let you = "You".to_string();
            names.insert(you.clone());
            for name in names.iter() {
                rules.insert((name.clone(), you.clone()), 0);
                rules.insert((you.clone(), name.clone()), 0);
            }
        }
        let arrangement = Self::search(&rules, &names, |best, curr| curr > best);
        // println!("{:?}: {:?}", arrangement.seats, arrangement.cost.unwrap());

        arrangement.cost.unwrap()
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY13_EXAMPLE, 330),
                (self.input(), 664),
            ],
            test_cases![
                // (DAY13_EXAMPLE, 0),
                (self.input(), 640),
            ]
        ]
    }
}

impl Default for Day13 {
    fn default() -> Self {
        Self::new()
    }
}

struct SeatingArrangement {
    cost: Option<isize>,
    seats: Vec<String>,
}

impl Day13 {
    pub fn new() -> Self {
        Self {
        }
    }

    fn search(costs: &FxHashMap<(String, String), isize>, items: &FxHashSet<String>, comparer: fn(current_best: isize, cost: isize) -> bool) -> SeatingArrangement {
        let mut best: SeatingArrangement = SeatingArrangement {cost: None, seats: vec![]};

        for item in items {
            let mut to_visit = items.clone();
            to_visit.remove(item);
            
            Self::search_(costs, &to_visit, &mut best, vec![item.clone()], comparer);
        }
        

        best
    }
    
    fn search_(costs: &FxHashMap<(String, String), isize>, unvisited: &HashSet<String, FxBuildHasher>, best: &mut SeatingArrangement, curr_route: Vec<String>, comparer: fn(current_best: isize, cost: isize) -> bool) {
        for curr in unvisited {
            let mut next_unvisited = unvisited.clone();
            next_unvisited.remove(curr);
            let mut next_route = curr_route.clone();
            next_route.push(curr.clone());

            if next_unvisited.is_empty() {
                let cost = Self::route_cost(costs, &next_route);
                if best.cost.is_none_or(|x| comparer(x, cost)) {
                    best.seats = next_route.clone();
                    best.cost = Some(cost);
                }

                return;
            } else {
                Self::search_(costs, &next_unvisited, best, next_route, comparer)
            }
        }
    }

    fn route_cost(graph: &FxHashMap<(String, String), isize>, route: &[String]) -> isize {
        // println!("route: {:?}", route);
        let forward: isize = route.iter()
            .circular_tuple_windows().take(route.len()+1)
            .map(|(from, to)| graph.get(&(from.clone(), to.clone())).unwrap())
            .sum();
        let backward: isize = route.iter().rev()
            .circular_tuple_windows().take(route.len()+1)
            .map(|(from, to)| graph.get(&(from.clone(), to.clone())).unwrap())
            .sum();
        forward + backward
    }
}
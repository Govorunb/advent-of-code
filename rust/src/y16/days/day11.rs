use pathfinding::prelude::{astar, dijkstra};

use crate::*;

aoc_day!(
    day = 11,
    output = usize,
    examples = [
"The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant."
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 11),
            // (Self::INPUT, 31), // 1.3sec...
        ],
        test_cases![
            // (Self::EXAMPLES[0], 0),
            // (Self::INPUT, 55), // 70sec...
        ]
    ],
    solve = |input, part| {
        let lines = input.lines();
        let r = Regex::new(r#"an? (?<gen>\w+) generator|an? (?<chip>\w+)-compatible microchip"#).unwrap();
        let items = lines.enumerate().map(|(i, l)| r.captures_iter(l)
            .map(|c| {
                if let Some(m_rtg) = c.name("gen") {
                    (Item::Gen(m_rtg.string()), i)
                } else {
                    (Item::Chip(c.string("chip")), i)
                }
            }).collect_vec()
        ).collect_vec();
        let mut state = State {
            elevator: 0,
            items: items.into_iter().flatten().collect_vec()
        };
        match part {
            Part::One => {},
            Part::Two => {
                for mat in ["elerium", "dilithium"] {
                    state.items.push((Item::Gen(mat.into()),0));
                    state.items.push((Item::Chip(mat.into()),0));
                }
            }
        };
        let mut total_states = 0;
        let mut expanded_states = 0;
        // TODO
        // gen-chip pair can be brought up 3 floors in 12 steps
        // let's say 1F has items Aa Bb
        //   Aa up 3 floors, a down 3
        //   ab up 3, b down 3
        // kind of backwards "proven" by p2 adding 24 steps
        // just need to get to a state where everything is paired at 1F (or higher)
        // then it's easy :)
        let res = astar(&state,
            |s: &State| {
                expanded_states += 1;
                s.successors().into_iter().inspect(|_| total_states += 1).collect_vec()
            }, // re-collect for weirdo lifetime
            |s: &State| s.heuristic(),
            |s: &State| s.goal()
        );

        println!("total {total_states} states... expanded {expanded_states}");

        res.unwrap().1
    }
);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Item {
    Chip(String),
    Gen(String),
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    elevator: usize,
    items: Vec<(Item, usize)>,
}

impl State {
    pub fn successors(&self) -> impl IntoIterator<Item = (State, usize)> {
        [-1isize, 1].iter().flat_map(move |&dir| {
            let new_floor = self.elevator as isize + dir;
            if new_floor < 0 || new_floor > 3 {
                return vec![];
            }
            let new_floor = new_floor as usize;
            let movable = self.items.iter()
                .enumerate()
                .filter(|&(_, &(_, f))| f == self.elevator)
                .map(|(i, _)| i);
            // going down should prefer moving 1 item, going up should prefer moving 2 items
            let moves = if dir < 0 {[1, 2usize]} else {[2usize, 1]};
            moves.iter().flat_map(|&move_num_items| {
                movable.clone()
                    .combinations(move_num_items)
                    .filter_map(move |v| {
                        let mut new_state = self.clone();
                        new_state.elevator = new_floor;
                        for i in v {
                            new_state.items[i].1 = new_floor;
                        }
                        Some(new_state)
                    })
                    .filter(|s| s.valid())
                    .map(move |s| (s,1)) // move cost
            }).collect_vec()
        })
    }

    fn chips(&self) -> impl Iterator<Item = (&String, usize)> {
        self.items.iter()
            .filter_map(|(i, f)| match i {
                Item::Chip(c) => Some((c, *f)),
                _ => None,
            })
    }
    fn gens(&self) -> impl Iterator<Item = (&String, usize)> {
        self.items.iter()
            .filter_map(|(i, f)| match i {
                Item::Gen(g) => Some((g, *f)),
                _ => None,
            })
    }

    pub fn valid(&self) -> bool {
        'chip: for (c, cf) in self.chips() {
            let mut wrong = false;
            for (g, gf) in self.gens() {
                if cf != gf {continue};
                if c == g {continue 'chip}; // safe
                wrong = true;
            }
            if wrong {
                return false;
            }
        }
        return true;
    }

    pub fn goal(&self) -> bool {
        self.elevator == 3
        && self.items.iter().all(|g| g.1 == 3)
    }
    pub fn heuristic(&self) -> usize {
        (3 - self.elevator)
        + self.items.iter().map(|g| 3 - g.1).sum::<usize>()
    }
}
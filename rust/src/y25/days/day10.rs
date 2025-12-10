use std::io::stdin;
use std::ops::Add;

use pathfinding::prelude::{astar, dijkstra};
use z3::{Optimize, Params, SatResult, Solver};
use z3::ast::*;

use crate::*;

aoc_day!(
    day = 10,
    output = usize,
    examples = [
"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 7),
            (Self::INPUT, 375),
        ],
        test_cases![
            (Self::EXAMPLES[0], 33),
            // (Self::INPUT, 0),
        ]
    ],
    solve = |input, part| {
        let manual = input.lines()
            .map(|l| {
                let mut sp = l.split_ascii_whitespace();
                let s_lights = sp.next().unwrap().trim_matches(['[',']']);
                let s_jolt = sp.next_back().unwrap().trim_matches(['{','}']);
                let s_wiring = sp;
                
                let lights = s_lights.chars().map(|c| matches!(c, '#')).collect_vec();
                let wiring = s_wiring.map(|s| s.trim_matches(['(',')'])
                        .split(',')
                        .map(|d| d.parse::<usize>().unwrap()).collect_vec()
                ).collect_vec();
                let jolt = s_jolt.split(',').map(|d| d.parse::<usize>().unwrap()).collect_vec();
                Machine {
                    lights,
                    buttons: wiring,
                    joltage: jolt,
                }
            }).collect_vec();
        match part {
            Part::One => {
                manual.iter().map(|machine| {
                    let start_lights = vec![false; machine.lights.len()];
                    let x = astar(&start_lights, |l| {
                        machine.buttons.iter().map(move |b| {
                            let mut next = l.clone();
                            for &i in b {
                                next[i] = !next[i];
                            }
                            (next, 1)
                        }).collect_vec().into_iter()
                    }, |l| {
                        machine.lights.iter().zip(l.iter())
                            .map(|(a,b)| a != b)
                            .count()
                    }, |l| machine.lights == *l);
                    x.unwrap().1
                }).sum::<usize>()
            },
            Part::Two => {
                manual.iter().enumerate().map(|(_i, machine)| {
                    // return p2_z3(&machine.buttons, &machine.joltage); // slower, phew

                    // println!("machine {_i}\n\tbuttons: {:?}\n\tjolts: {:?}", machine.buttons, machine.joltage);

                    let mut buttons = machine.buttons.clone();
                    let mut joltage = machine.joltage.clone();
                    let mut total = 0;

                    while let Some((next_buttons, next_joltage, presses)) = p2_simplify(&buttons, &joltage) {
                        // println!("\tsimplified away {presses} presses");
                        // println!("\t\t{next_buttons:?}");
                        // println!("\t\t{next_joltage:?}");
                        buttons = next_buttons;
                        joltage = next_joltage;
                        total += presses;
                        // confirm_stdin();
                    }
                    // if total > 0 {
                    //     println!("simplified:\n\tbuttons: {:?}\n\tjolts: {:?}", buttons, joltage);
                    // }
                    total + p2_z3(&buttons, &joltage)
                }).sum::<usize>()
            }
        }
    }
);

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn p2_astar_timeout(buttons: &[Vec<usize>], joltage: &Vec<usize>) -> usize {
    // we count down instead of up (it's funnier)
    let start_jolts = joltage.clone();
    astar(&start_jolts, |l| {
        let mut res = vec![];
        res.extend(buttons.iter()
            .flat_map(|b| {
                let max = b.iter().map(|&i| l[i]).min().unwrap();
                (1..=max).rev() // reverse so we don't waste 10 million years pressing buttons once
                    .map(move |presses| {
                    let mut next = l.clone();
                    for &i in b {
                        next[i] -= presses;
                    }
                    (next, max)
                })
            })
        );
        res.into_iter()
    }, |l| *l.into_iter().max().unwrap(), |l| l.iter().all(|&j| j == 0))
    .unwrap().1
}

fn p2_disjoint_too_low(buttons: &[Vec<usize>], joltage: &[usize]) -> usize {
    if buttons.is_empty() || joltage.is_empty() {
        return 0;
    }
    let mut disjoint = vec![];
    let max_i = joltage.iter().position_max().unwrap();
    disjoint.push(max_i);
    for i in 0..joltage.len() {
        if joltage[i] == 0 {continue;}
        if buttons.iter().any(|b| b.contains(&i) && disjoint.iter().any(|d| b.contains(d))) {
            continue;
        }
        // disjoint from max, needs its own presses
        disjoint.push(i);
    }

    let dj_jolt = disjoint.iter().map(|d| joltage[*d]).collect_vec();

    println!("\tdj: {disjoint:?} -> {dj_jolt:?}");

    let total = dj_jolt.into_iter().sum::<usize>();
    println!("\ttotal: {total}");
    // confirm_stdin();
    total
}

fn p2_simplify(buttons: &[Vec<usize>], joltage: &[usize]) -> Option<(Vec<Vec<usize>>, Vec<usize>, usize)> {
    let buttons_per_counter = (0..joltage.len())
        .map(|i| buttons.iter().filter(|b| b.contains(&i)).collect_vec())
        .collect_vec();
    let mut remove = FxHashSet::default();
    for (_counter, b) in buttons_per_counter.iter().enumerate() {
        if b.len() == 1 {
            // println!("\t{_counter}: {b:?}");
            remove.insert(b[0]);
        }
    }
    if remove.is_empty() {
        return None;
    }
    let mut next_buttons = buttons.to_vec();
    let mut next_jolts = joltage.to_vec();
    let mut presses = 0;
    for b in remove {
        next_buttons.retain(|e| e != b);
        let min = b.iter().map(|&i| next_jolts[i]).min().unwrap();
        // println!("{b:?} {:?}", b.iter().map(|&i| next_jolts[i]).collect_vec());
        for j in b {
            next_jolts[*j] -= min;
        }
        presses += min;
    }
    
    // TODO: also remove from middle (affects button indices)
    while next_jolts.pop_if(|j| *j == 0).is_some() { }
    
    Some((next_buttons, next_jolts, presses))
}

// this used to be named "p2_shameful_z3"
// but deciphering z3 docs is a challenge in its own right
// so i declare "sufficient effort/investment" has been achieved
fn p2_z3(buttons: &[Vec<usize>], joltage: &[usize]) -> usize {
    let no_buttons = buttons.is_empty();
    let no_joltage = joltage.is_empty();
    assert!(no_buttons == no_joltage, "idiot {buttons:?} {joltage:?}");
    if no_buttons || no_joltage {return 0};

    let solver = Optimize::new();

    let vars = (0..buttons.len())
        .map(|i| Int::fresh_const(format!("b{i}").as_str()))
        .collect_vec();
    for v in &vars {
        solver.assert(&v.ge(0));
    }
    // sum(vars[bi] == j)
    for (ji, &j) in joltage.iter().enumerate() {
        if j == 0 {continue};
        let buttons_sum = z3_sum(buttons.iter().enumerate()
            .filter_map(|(bi, b)| b.contains(&ji).then(|| vars[bi].clone()))
        );
        solver.assert(&buttons_sum.eq(Int::from_u64(j as u64)));
    }

    let sum = z3_sum(vars.iter().cloned());
    solver.minimize(&sum);
    
    assert!(solver.check(&[]) == SatResult::Sat, "idiot 2: the booger");
    let m = solver.get_model().unwrap();
    vars.iter()
        .map(|v| m.eval(v, true)
            .and_then(|i| i.as_u64()).unwrap() as usize
        ).sum()
}

fn z3_sum(ints: impl Iterator<Item = Int>) -> Int {
    ints.reduce(|a, b| a.add(b)).unwrap()
}

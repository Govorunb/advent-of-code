use std::str::FromStr;
use std::sync::LazyLock;
use itertools::multizip;
use pathfinding::prelude::{astar, astar_bag, dijkstra};
use crate::*;

aoc_day!(
    day = 21,
    output = usize,
    examples = ["029A\n980A\n179A\n456A\n379A"],
    tests = [
        test_cases![
            ("029A", 68*29),
            ("980A", 60*980),
            ("179A", 68*179),
            ("456A", 64*456),
            ("379A", 64*379),
            (Self::EXAMPLES[0], 126384),
            (Self::INPUT, 155252),
        ],
        test_cases![
            // (Self::EXAMPLES[0], 0),
            // (Self::INPUT, 0),
        ]
    ],
    solve = |input, part| {
        let codes = input.lines()
            .map(|l| l.to_string())
            .collect_vec();
        let layers = match part {
            Part::One => 3,  // (you +) 2 keypads + numpad
            Part::Two => 26, // (you +) 25 keypads + numpad
        };
        // let you = codes.iter().map(|code| buh(code, layers));
        codes.iter()
            .map(|code| {
                let seq = key_sequence(&NUMPAD, code.clone(), layers);
                println!("{code} | {seq}");
                seq
            })
            .map(|seq| expand_layers(seq, layers-1))
            .zip(&codes)
            .map(|(seq, code)| {
                let code_i = code[..code.len()-1].parse::<usize>().unwrap();
                let i = seq.len() * code_i;
                println!("{code}: ({}*{code_i}={i}) {seq:?}", seq.len());
                i
            }).sum()
    }
);

static NUMPAD: LazyLock<Grid<char>> = LazyLock::new(|| Grid::from_str("789\n456\n123\n 0A").unwrap());
static KEYPAD: LazyLock<Grid<char>> = LazyLock::new(|| Grid::from_str(" ^A\n<v>").unwrap());
static mut DP: LazyLock<FxHashMap<String, Vec<String>>> = LazyLock::new(FxHashMap::default);
static mut DP2: LazyLock<FxHashMap<(char, char), String>> = LazyLock::new(FxHashMap::default);

fn key_sequence(pad: &Grid<char>, code: String, layers: usize) -> String {
    // println!("{code}");
    let chars = ['A'].into_iter().chain(code.chars());
    let keypad_dir = None; // always starts at A; keypad subsequences always end at A
    let mut out = String::new();
    for (from, to) in chars.tuple_windows() {
        let str = if pad == &*KEYPAD {
            unsafe {
                #[allow(static_mut_refs)] // brother PLEASE i promise you it's fine
                LazyLock::force_mut(&mut DP2)
                    .entry((from, to))
                    .or_insert_with(|| best_keypad_move(pad, from, to, keypad_dir, layers))
                    .clone()
            }
        } else {
            best_keypad_move(pad, from, to, keypad_dir, layers)
        };
        out.push_str(&str);
    }

    // println!("code {code}: {out}");

    out
}

fn best_keypad_move(pad: &Grid<char>, from: char, to: char, curr_dir: Option<Direction>, layers: usize) -> String {
    if layers == 0 {
        return to.to_string();
    }

    if from == to {
        // no movement needed
        return "A".to_string();
    }

    let from_pos = pad.find(&from).unwrap();
    let to_pos = pad.find(&to).unwrap();
    let delta = to_pos - from_pos;
    if from_pos.manhattan_distance(to_pos) == 1 {
        let dir = Direction::try_from(to_pos - from_pos).unwrap();
        return dir.to_string() + "A";
    }
    if delta.x == 0 || delta.y == 0 {
        let dir = Direction::try_from(delta).unwrap();
        return dir.to_string().repeat((delta.x.abs() + delta.y.abs()) as usize) + "A";
    }

    // let cost_next_layer = |curr_dir: Option<Direction>, dir: Direction| {
    //     let curr_char = curr_dir.map_or("A".to_string(), |dir| dir.to_string()).chars().next().unwrap();
    //     let to_char = dir.to_string().chars().next().unwrap();
    //     best_keypad_move(&KEYPAD, curr_char, to_char, curr_dir, layers - 1).len()
    // };
    let succ = |(pt, curr_dir): &(Vector2, Option<Direction>)| {
        pt.adjacent()
            .filter(|n| pad.get(n).is_some_and(|&c| c != ' '))
            .map(|n| {
                let dir = Direction::try_from(n - *pt).unwrap();
                let cost = if *curr_dir != Some(dir) { 10 } else { 1 };
                // let next_cost = cost_next_layer(*curr_dir, dir);
                ((n, Some(dir)), cost)// * next_cost
            })
            .collect_vec()
            .into_iter()
    };
    let (paths, _cost) = astar_bag(&(from_pos, curr_dir), succ, |_| 0, |&(p, _)| p == to_pos).unwrap();
    let presses = paths.map(|path| {
        path.par_iter().skip(1)
            .map(|&(_, od)| od.expect("pressed A to move!"))
            .map(|dir| dir.to_string())
            .chain(["A".to_string()])
            .collect::<String>()
    }).collect_vec();
    let expanded = presses.iter()
        .map(|s| expand_layers(s.clone(), layers - 1))
        .collect_vec();
    let i_best = expanded.iter().position_min_by_key(|s| s.len()).unwrap();
    let best_path = presses[i_best].clone();
    let cost = best_path.len();
    if layers >= 15 {
        println!("\nfrom {from} to {to} at {curr_dir:?} ({layers}) -> ({cost}) {best_path}");
    }
    best_path
}

fn expander(path: String, layers: usize) -> impl Iterator<Item = String> {
    let mut i = layers;
    std::iter::successors(Some(path.clone()), move |curr| {
        // assuming that path is a keypad string (<A^A^^>AvvA) and not a numpad string (029A)
        (i > 0).then(|| {
            let res = key_sequence(&KEYPAD, curr.clone(), i);
            i -= 1;
            res.clone()
        })
    }).skip(1)
}

fn expand_layers(path: String, layers: usize) -> String {
    if layers == 0 {return path}
    // theoretically you can split the big string on 'A's
    // since, to press 'A' on any layer, every layer below must also be on 'A'
    // (all the way to the bottom, where your layer must be pressing 'A')
    // but in practice i couldn't get an implementation working that didn't massively slow everything down
    unsafe {
        #[allow(static_mut_refs)] // brother PLEASE i promise you it's fine
        let x = LazyLock::force_mut(&mut DP);
        let y = x.entry(path.clone()).or_insert_with(|| expander(path, layers).collect_vec());
        if y.len() < layers {
            let remaining = layers - y.len();
            let latest = y.last().unwrap();
            y.extend(expander(latest.clone(), remaining));
        }
        (*y)[layers - 1].clone()
    }
}

// unused solution below this line
// it is theoretically sound but practically VERY incorrect as it runs in something like O(n^O(n))

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    robot_keypads: Vec<Vector2>,
    robot_presses: Vec<String>,
    your_presses: String, // your keypad is stateless
}

fn buh(code: &str, layers: usize) -> Vec<String> {
    let mut keypads = vec![KEYPAD.find(&'A').unwrap(); layers-2];
    keypads.push(NUMPAD.find(&'A').unwrap());
    let state = State {
        robot_keypads: keypads,
        robot_presses: vec![String::new(); layers-1],
        your_presses: String::new(),
    };
    // cost should always be higher than heuristic, so...
    let cost_per_move = 100usize.pow(layers as u32);
    let succ = |state: &State| {
        KEYPAD.elements()
            .filter(|&&c| c != ' ')
            .filter_map(|&c| state.clone().press(c, 0).map(|st| (st, cost_per_move)))
            .collect_vec().into_iter()
    };
    let heur = |state: &State| {
        let Some(remaining) = code.strip_prefix(state.numpad_presses())
            else {return usize::MAX};
        let mut cost = 100 * remaining.len();
        let next_pos = NUMPAD.find(&remaining.chars().next().unwrap()).unwrap();
        let dist_numpad = state.numpad().manhattan_distance(next_pos);
        cost += dist_numpad;
        for layer in state.robot_keypads.iter().rev().skip(1) {
            cost *= 10;
            if dist_numpad == 0 {
                // on the right button - need to move all lower layers to 'A'
                cost += layer.manhattan_distance(KEYPAD.find(&'A').unwrap());
            }
        }

        cost
    };
    let goal = |state: &State| state.numpad_presses() == code;
    let (solns, cost) = astar_bag(&state, succ, heur, goal).unwrap();

    let all = solns.map(|soln| soln.last().unwrap().your_presses.clone()).collect_vec();
    println!("{cost}: {all:?}");
    all
}

impl State {
    pub fn press(&self, btn: char, layer: usize) -> Option<Self> {
        // so much off by 1 crap...
        // layer 0 - you, not in robot_presses or robot_keypads obviously
        // layer 1 (index 0) - keypad that is deepest/furthest from numpad
        // ...
        // layer N (index N-1) - numpad
        {
            if layer == self.robot_presses.len() {
                // there is no next layer so no need for any complicated processing
                debug_assert!(matches!(btn, '0'..='9' | 'A'), "invalid '{btn}' in numpad branch (layer {layer})");
                return Some(self.clone());
            }

            // if btn == ' ' {
            //     println!("{self:?}");
            //     debug_assert!(['<','>','^','v','A'].contains(&btn), "invalid '{btn}' in keypad branch (layer {layer})");
            // }

            let delta = Direction::parse(btn).map_or(Vector2::ZERO, |dir| dir.into());
            // curr index (layer-1) + 1
            let up_index = layer; // what's up index
            let up_layer = layer + 1; // what's up layer
            let up_pos = self.robot_keypads[up_index]; // what's up pos
            if delta == Vector2::ZERO {
                // 'A' - press button on next layer
                let up_pad = self.pad(up_layer); // what's up pad
                let up_btn = up_pad[up_pos]; // what's up btn
                self.press(up_btn, up_layer)
            } else {
                // directional press moves next layer up (e.g. layer 0 (you) moves layer 1)
                let next_up_pos = up_pos + delta;
                // in bounds + not 'space'
                self.pad(layer+1).get(&next_up_pos)
                    .and_then(|&c| (c != ' ').then_some(c))?;

                let mut next_state = self.clone();
                next_state.robot_keypads[up_index] = next_up_pos;
                Some(next_state)
            }
        }.map(|mut next| {
            if layer == 0 {
                next.your_presses.push(btn)
            } else {
                next.robot_presses[layer-1].push(btn);
            };
            next
        })
    }

    pub fn numpad(&self) -> Vector2 {*self.robot_keypads.last().unwrap()}
    pub fn numpad_presses(&self) -> &String {self.robot_presses.last().unwrap()}

    fn pad(&self, layer: usize) -> &Grid<char> {
        if layer == self.robot_presses.len() {
            &NUMPAD
        } else {
            &KEYPAD
        }
    }
}

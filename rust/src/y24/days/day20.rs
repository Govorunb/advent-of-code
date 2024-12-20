use std::str::FromStr;
use pathfinding::prelude::{astar, dfs, dijkstra};
use crate::*;

aoc_day!(
    day = 20,
    output = usize,
    examples = [
"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 1), // total 44
            (Self::INPUT, 1327), // total 7038
        ],
        test_cases![
            (Self::EXAMPLES[0], 32+31+29+39+25+23+20+19+12+14+12+22+4+3), // total 3081
            (Self::INPUT, 985737), // total 1657732
        ]
    ],
    solve = |input, part| {
        let grid: Grid<char> = Grid::from_str(input).unwrap();
        let start = grid.find(&'S').unwrap();
        let end = grid.find(&'E').unwrap();
        
        let is_example = grid.width() < 20;
        let threshold = if is_example {50} else {100};
        
        let allowed_cheat_dist = match part {
            Part::One => 2,
            Part::Two => 20,
        };
        let succ_path = |p: &Vector2| {
            p.adjacent()
                .filter(|n| grid.get(n).is_some_and(|&c| c != '#'))
        };
        let base_path = dfs(start, succ_path, |&p| p == end).unwrap();
        
        let over_thresh = count_only_threshold(&base_path, allowed_cheat_dist, threshold);
        debug_assert_eq!(over_thresh, count_total(&base_path, allowed_cheat_dist, threshold));
        over_thresh
    }
);

fn count_total(path: &[Vector2], allowed_cheat_dist: usize, threshold: usize) -> usize {
    let mut target = 0;
    let mut counts: FxHashMap<usize, usize> = FxHashMap::default();
    for (i, &p) in path.iter().enumerate()
    {
        for (j, &cheatable) in path.iter().enumerate()
            // 4 is the minimum distance that makes sense
            // (you have to skip at least 1 wall, which takes min 2 picoseconds)
            .skip(i+3)
        {
            let dist = cheatable.manhattan_distance(p);
            // manhattan dist is minimum so if we're over that there can't possibly be a path
            if dist > allowed_cheat_dist { continue }

            let regular_cost = j - i;
            if regular_cost <= dist { continue }
            let saves = regular_cost - dist;

            *counts.entry(saves).or_default() += 1;
            if saves >= threshold { target += 1; }
        }
    }
    let total: usize = counts.values().sum();
    println!("total: {total}");

    target
}

fn count_only_threshold(path: &[Vector2], allowed_cheat_dist: usize, threshold: usize) -> usize {
    let mut target = 0;
    for (i, &from) in path.iter().enumerate() {
        for (j, &to) in path.iter().enumerate().rev()
            // .take_while(|&(j, _)| j > i) // not needed for example/input
        {
            let regular_cost = j - i;
            if regular_cost < threshold { break } // can't possibly save over threshold
            
            let dist = from.manhattan_distance(to);
            // manhattan dist is minimum so if we're over that there can't possibly be a path
            if dist > allowed_cheat_dist { continue }
            
            if regular_cost <= dist { continue }
            let saves = regular_cost - dist;

            if saves >= threshold { target += 1; }
        }
    }

    target
}
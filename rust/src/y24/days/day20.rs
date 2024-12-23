use std::str::FromStr;
use pathfinding::prelude::dfs_reach;
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
        // let end = grid.find(&'E').unwrap(); // there's only one path
        
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
        let base_path = dfs_reach(start, succ_path).collect_vec();

        // let x = count_total(&base_path, allowed_cheat_dist as usize, threshold);
        count_only_threshold(grid.size(), &base_path, allowed_cheat_dist, threshold)
    }
);

fn count_total(path: &[Vector2], allowed_cheat_dist: usize, threshold: usize) -> usize {
    let mut target = 0;
    let mut counts: FxHashMap<usize, usize> = FxHashMap::default();
    // let mut cheats76: Vec<(Vector2, Vector2)> = vec![];
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
            // if saves == 76 {
            //     cheats76.push((p, cheatable));
            // }

            *counts.entry(saves).or_default() += 1;
            if saves >= threshold { target += 1; }
        }
    }
    // println!("counts: {counts:?}");
    let total: usize = counts.values().sum();
    println!("total: {total}");
    // if cheats76.len() == 3 {
    //     println!("76: {cheats76:?}");
    // }

    target
}

fn count_only_threshold(grid_size: Size, path: &[Vector2], allowed_cheat_dist: isize, threshold: usize) -> usize {
    // let distances = FxHashMap::from_iter(path.iter().enumerate().map(|(i, &p)| (p,i)));
    // faster than a hashmap of path points, uses a bit more memory though
    let mut distances = Grid::from_origin(grid_size).unwrap();
    for (i, &p) in path.iter().enumerate() {
        distances[p] = Some(i);
    }
    path.par_iter().enumerate().rev()
        // if we're 9ps away from end, we can't possibly save 10ps
        .take(path.len()-threshold)
        .map(|(j, &to)| {
            // O(n^2)
            // [---->i....|___________j<----]
            //  ^checking^ ^threshold^
            // path.iter().enumerate()
            //     .take(j-threshold)
            //     .filter(move |&(i, &from)| {
            //         let dist = from.manhattan_distance(to);
            //         // manhattan dist is minimum so if we're over, there can't possibly be a path
            //         if dist > allowed_cheat_dist {return false}
            //         
            //         let regular_cost = j - i;
            //         regular_cost.checked_sub(dist)
            //             .is_some_and(|saves| saves >= threshold)
            //     })
            //     .count()
            
            // O(n*d^2)
            (-allowed_cheat_dist..=allowed_cheat_dist)
                .filter(|dy| distances.bounds().y_range().contains(&(to.y + dy)))
                .flat_map(|dy| {
                    let max_dx = allowed_cheat_dist - dy.abs();
                    (-max_dx..=max_dx).map(move |dx| (dx,dy))
                }).filter(|&(dx, dy)| {
                    let pt = to + Vector2::from((dx, dy));
                    let Some(i) = distances.get(&pt).and_then(|&d| d)
                        else {return false};
            
                    if i + threshold >= j {return false} // not enough distance to save
            
                    let dist = (dx.abs() + dy.abs()) as usize;
            
                    let regular_cost = j - i;
                    let saves = regular_cost - dist;
                    saves >= threshold
                }).count()
        }).sum()
}

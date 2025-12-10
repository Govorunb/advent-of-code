use std::ops::RangeInclusive;
use pathfinding::prelude::dijkstra_reach;

use crate::*;


aoc_day!(
    day = 9,
    output = usize,
    examples = [
"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 50),
            (Self::INPUT, 4771508457),
        ],
        test_cases![
            (Self::EXAMPLES[0], 24),
            // (Self::INPUT, 0),
        ]
    ],
    solve = |input, part| {
        let lines = input.lines();
        let pts: Vec<Vector2> = lines.map(|l| l.split_once(',').unwrap().map(|s| s.parse::<usize>().unwrap()))
            .map_into()
            .collect_vec();
        match part {
            Part::One => {
                pts.into_iter()
                    .pairwise()
                    .map(|(tl, br)| Rect::from_corners(tl, br).map(|r| r.area()).unwrap_or(0))
                    .max().unwrap()
            },
            Part::Two => {
                let bounds: Vec<Line> = pts.iter()
                    // .cycle().tuple_windows().take(pts.len())
                    .tuple_windows().chain(std::iter::once((&pts[pts.len()-1], &pts[0])))
                    .map(|(&a,&b)| Line::new(a, b).unwrap())
                    .collect_vec();

                // let (min_x, max_x) = pts.iter().map(|p| p.x).minmax().into_option().unwrap();
                // let (min_y, max_y) = pts.iter().map(|p| p.y).minmax().into_option().unwrap();

                // // preheat cpu to 95C
                // let vert_bounds_by_row = (0..=max_y)
                //     .map(|y| bounds.iter()
                //         .filter(|b| b.0.start() == b.0.end()) // vertical
                //         .filter(|b| b.1.contains(&y))
                //         .collect_vec()
                //     ).collect_vec();
                
                // let horiz_bounds_by_col = (0..=max_x)
                //     .map(|x| bounds.iter()
                //         .filter(|b| b.1.start() == b.1.end())
                //         .filter(|b| b.0.contains(&x))
                //         .collect_vec()
                //     ).collect_vec();

                let mut max_area = 0;
                for r in pts.iter().pairwise().map(|(&a, &b)| Rect::from_corners(a, b).unwrap()) {
                    if r.area() <= max_area {continue};
                    if let Some(inner) = r.inset(1) {
                        // pt inside - bad
                        // one of two sides over perimeter is outside
                        // if inner contains a pt, we contain both sides
                        if pts.iter().any(|p| inner.contains(p)) {continue};
                        // edge inside - same thing (we contain both inside and outside)
                        #[allow(deprecated)] {
                            if bounds.iter().any(|b| inner.intersects(b)) {continue};
                        }
                    }
                    
                    // here comes the fun part
                    // let corners = vec![
                    //     r.top_left(),
                    //     r.top_left() + Vector2::RIGHT * (r.width()-1),
                    //     r.top_left() + Vector2::DOWN * (r.height()-1),
                    //     r.bottom_right()
                    // ];
                    // // println!("\n{}", PrintVec(corners.clone()));
                    // if corners.iter().all(|p| check_bounds_wrong(&bounds, p, &vert_bounds_by_row, &horiz_bounds_by_col)) {
                    // // if corners.iter().all(|p| check_bounds_expensive_also_wrong(&bounds, p)) {
                    //     max_area = r.area();
                    //     println!("yippee: {}|{}: {}", r.top_left(),r.bottom_right(), r.area())
                    // }
                    
                    // ??????????????????????
                    max_area = r.area();
                };
                max_area
            }
        }
    }
);

type Bounds = (RangeInclusive<isize>, RangeInclusive<isize>);


fn cringe<T>(a: T, b: T) -> RangeInclusive<T> 
where T : PartialOrd
{
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

fn on_bounds<'a>(bounds: &'a Vec<Bounds>, p: &Vector2) -> impl Iterator<Item = &'a Bounds> {
    bounds.iter()
        .filter(|(rx, ry)|
            rx.contains(&p.x)
            && ry.contains(&p.y)
        )
}

fn on_any_bounds(bounds: &Vec<Bounds>, p: &Vector2) -> bool {
    on_bounds(bounds, p).any(|_| true)
}

fn cross_bounds_y(bounds: &Vec<Bounds>, p: &Vector2) -> bool {
    bounds.iter()
        .filter(|b| b.1.clone().count() > 1)
        .any(|(rx, ry)|
            rx.contains(&p.x)
            && ry.contains(&p.y)
        )
}

fn check_bounds_wrong(bounds: &Vec<Bounds>, p: &Vector2, vbounds_by_row: &Vec<Vec<&Bounds>>, hbounds_by_col: &Vec<Vec<&Bounds>>) -> bool {
    if on_any_bounds(&bounds, p) {
        return true;
    }
    if p.x as usize > hbounds_by_col.len() || p.y as usize > vbounds_by_row.len() {
        return false;
    }
    
    let (vert_crossed, vert_uncrossed) = vbounds_by_row[p.y as usize].iter()
        .cloned().cloned()
        .partition(|b| b.0.start() < &p.x)
        .map(|c: Vec<Bounds>| c.len());
    let (_horiz_crossed, horiz_uncrossed) = hbounds_by_col[p.x as usize].iter()
        .cloned().cloned()
        .partition(|b| b.1.start() < &p.y)
        .map(|c: Vec<Bounds>| c.len());
    // let vert_makes_sense = (vert_uncrossed + vert_crossed) % 2 == 0;
    // let horiz_makes_sense = (horiz_uncrossed + _horiz_crossed) % 2 == 0;

    vert_crossed > 0 && vert_uncrossed > 0
    && _horiz_crossed > 0 && horiz_uncrossed > 0
}

fn check_bounds_expensive_also_wrong(bounds: &Vec<Bounds>, p: &Vector2) -> bool {
    // https://en.wikipedia.org/wiki/Point_in_polygon
    if on_any_bounds(bounds, p) {
        return true;
    }
    let mut inside = false;
    let start = Vector2 {x: 1, y: p.y};
    for p in start.ray(Direction::East.into()).take(1 + p.x as usize) {
        // possible to glide along an edge (does not count as crossing)
        if !on_any_bounds(bounds, &p) {
            let prev = p + Vector2::LEFT;
            let prev_bounds = on_bounds(bounds, &prev);
            match prev_bounds.count() {
                0 => continue,
                1 => inside = !inside,
                2 => continue,
                _ => unreachable!()
            }
        }
    }
    inside
}

/* hello again y23d10

*/


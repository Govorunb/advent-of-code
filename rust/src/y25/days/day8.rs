use std::iter;
use crate::*;

aoc_day!(
    day = 8,
    output = usize,
    examples = [
"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 40),
            (Self::INPUT, 79056),
        ],
        test_cases![
            (Self::EXAMPLES[0], 25272),
            (Self::INPUT, 4639477),
        ]
    ],
    solve = |input, part| {
        let lines = input.lines();
        let pts: Vec<Vector3> = lines.map(|l| l.split(',')
            .map(|c| c.parse::<isize>().unwrap())
            .next_tuple()
            .map(|(x,y,z)| Vector3 {x,y,z})
            .unwrap()
        ).collect_vec();
        let pairs = pts.clone().into_iter()
            .pairwise()
            .sorted_by_cached_key(|(a,b)| euclid_dist_sqr_3d(*a,*b));
        let mut circuits = pts.into_iter()
            .map(|p| vec![p])
            .collect_vec();
        match part {
            Part::One => {
                // completely diabolical to have different KINDS of cutoffs for the example and the actual input
                let cables = if input.lines().count() < 100 {9} else {999};
                let mut connected = 0;
                for (a, b) in pairs.take(1000) {
                    if connect_pair(&mut circuits, (a,b)) {
                        connected += 1;
                        if connected >= cables {
                            break;
                        }
                    }
                };
                circuits.iter()
                    .map(|c| c.len())
                    .sorted().rev()
                    .take(3)
                    .product::<usize>()
            },
            Part::Two => {
                for (a, b) in pairs {
                    connect_pair(&mut circuits, (a,b));
                    if circuits.len() == 1 {
                        return (a.x * b.x) as usize;
                    }
                }
                unreachable!()
            }
        }
    }
);

fn euclid_dist_sqr_3d(a: Vector3, b: Vector3) -> u128 {
    (b.x - a.x).pow(2) as u128
        + (b.y - a.y).pow(2) as u128
        + (b.z - a.z).pow(2) as u128
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Vector3 {
    x: isize,
    y: isize,
    z: isize
}

fn connect_pair(circuits: &mut Vec<Vec<Vector3>>, pair: (Vector3, Vector3)) -> bool {
    let (a,b) = &pair;
    let c_has_0 = circuits.iter()
        .find_position(|c| c.contains(a))
        .map(|(i, _)| i);
    let c_has_1 = circuits.iter()
        .find_position(|c| c.contains(b))
        .map(|(i, _)| i);
    match (c_has_0, c_has_1) {
        (None, None) => circuits.push(vec![*a, *b]),
        (Some(c0), None) => circuits[c0].push(*b),
        (None, Some(c1)) => circuits[c1].push(*a),
        (Some(c0), Some(c1)) => {
            if c0 == c1 {
                return false;
            }
            let h1 = circuits[c1].clone(); // borrowing
            circuits[c0].extend(h1.iter());
            circuits.swap_remove(c1);
        },
    }
    true
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

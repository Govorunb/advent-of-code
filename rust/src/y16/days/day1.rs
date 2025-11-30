use crate::*;

aoc_day!(
    day = 1,
    output = usize,
    examples = [
        "R2, L3",
        "R2, R2, R2",
        "R5, L5, R5, R3",
        "R8, R4, R4, R8"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 5),
            (Self::EXAMPLES[1], 2),
            (Self::EXAMPLES[2], 12),
            (Self::INPUT, 278),
        ],
        test_cases![
            (Self::EXAMPLES[3], 4),
            (Self::INPUT, 161),
        ]
    ],
    solve = |input, part| {
        let instrs: Vec<Instruction> = input.split(", ").map(|s| s.trim()).map_into().collect_vec();
        let mut pos: Vector2 = (0, 0).into();
        let mut dir = Direction::North;
        match part {
            Part::One => {
                for instr in instrs {
                    dir = dir.turn(instr.turn);
                    pos += dir.to_vec2() * instr.walk;
                }
                pos.manhattan_distance(Vector2::ZERO)
            },
            Part::Two => {
                let mut visited = FxHashSet::default();
                for instr in instrs {
                    dir = dir.turn(instr.turn);
                    for p in pos.ray(dir.to_vec2()).take(instr.walk) {
                        if !visited.insert(p) {
                            return p.manhattan_distance(Vector2::ZERO)
                        }
                    }
                    pos += dir.to_vec2() * instr.walk;
                }
                unreachable!("did not visit any location twice")
            }
        }
    }
);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Instruction {
    turn: Turn,
    walk: usize,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        debug_assert!(value.len() > 0);
        let walk = value[1..].parse().unwrap();
        Instruction {
            turn: value.chars().next().unwrap().into(),
            walk
        }
    }
}

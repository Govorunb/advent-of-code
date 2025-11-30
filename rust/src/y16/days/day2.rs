use crate::*;

aoc_day!(
    day = 2,
    output = String,
    examples = [
"ULL
RRDDD
LURDL
UUUUD",
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], "1985".into()),
            (Self::INPUT, "48584".into()),
        ],
        test_cases![
            (Self::EXAMPLES[0], "5DB3".into()),
            (Self::INPUT, "563B6".into()),
        ]
    ],
    solve = |input, part| {
        let instr: Vec<Vec<Direction>> = input.lines()
            .map(|l| l.chars().map(to_dir).collect_vec())
            .collect_vec();
        let kpad = match part {
            Part::One => Grid::from_digits("123\n456\n789", 10),
            Part::Two => Grid::from_digits("00100\n02340\n56789\n0ABC0\n00D00", 16),
        };
        let mut pos: Vector2 = kpad.find(&5).unwrap();
        let mut code = String::with_capacity(instr.len());
        for dirs in instr {
            for dir in dirs {
                let new_pos = pos + dir.to_vec2();
                if let Some(d) = kpad.get(&new_pos) && *d != 0 {
                    pos = new_pos;
                }
            }
            let d = kpad.get(&pos).expect(&format!("Invalid pos {pos}, grid bounds {:?}", kpad.bounds()));
            code.push_str(&format!("{d:X}"));
        }
        code
    }
);

fn to_dir(c: char) -> Direction {
    match c {
        'U' => Direction::North,
        'D' => Direction::South,
        'L' => Direction::West,
        'R' => Direction::East,
        _ => unreachable!()
    }
}
use crate::*;

aoc_day!(
    day = 8,
    output = usize,
    examples = [
"rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 6),
            (Self::INPUT, 121),
        ],
        test_cases![
            // (Self::EXAMPLES[0], 0),
            // (Self::INPUT, 0),
        ]
    ],
    solve = |input, part| {
        let r = Regex::new(r#"rect (?<dx>\d+)x(?<dy>\d+)|rotate \w+ (?<rd>[xy])=(?<rt>\d+) by (?<ra>\d+)"#).unwrap();
        let instructions = r.captures_iter(input)
            .map(|c| {
                if let Some(_) = c.name("dx") {
                    Instruction::Draw(c.size("dx", "dy"))
                } else {
                    if let "x" = c.str("rd") {
                        Instruction::RotateCol(c.usize("rt"), c.usize("ra"))
                    } else {
                        Instruction::RotateRow(c.usize("rt"), c.usize("ra"))
                    }
                }
            });
        let mut grid = Grid::fill_with((50, 6).into(), ' ').unwrap();
        for i in instructions {
            i.exec(&mut grid);
        }
        match part {
            Part::One => {
                grid.elements().filter(|&&x| x == '#').count()
            },
            Part::Two => {
                // visual output
                if !cfg!(test) {
                    println!("{grid}");
                }
                0
            }
        }
    }
);

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
enum Instruction {
    Draw(Size),
    RotateCol(usize, usize),
    RotateRow(usize, usize),
}

impl Instruction {
    fn exec(self, grid: &mut Grid<char>) {
        // println!("Executing {self:?}");
        match self {
            Instruction::Draw(size) => {
                let rect = Rect::from_origin(size).unwrap();
                for p in rect {
                    grid[p] = '#';
                }
            },
            Instruction::RotateCol(col, amt) => {
                grid.slide_col(col, amt as isize);
            },
            Instruction::RotateRow(row, amt) => {
                grid.slide_row(row, amt as isize);
            },
        }
        // println!("Executed {self:?}");
        // println!("{grid}");
    }
}
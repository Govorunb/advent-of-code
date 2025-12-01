use num::ToPrimitive;

use crate::*;

aoc_day!(
    day = 4,
    output = usize,
    examples = [
"aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]"
],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 1514),
            (Self::INPUT, 185371),
        ],
        test_cases![
            // (Self::EXAMPLES[0], 0),
            (Self::INPUT, 984),
        ]
    ],
    solve = |input, part| {
        let lines = input.lines();
        let regex: Regex = Regex::new(r#"(?<n>[a-z\-]+)-(?<id>\d+)\[(?<c>[a-z]+)]"#).unwrap();

        let rooms = lines.map(|l| regex.captures(l).unwrap())
            .map(|m| Room {
                id: m.usize("id"),
                checksum: m.string("c"),
                name: m.string("n")
            })
            .collect_vec();
        let valid_rooms = rooms.iter().filter(|r| r.check());
        match part {
            Part::One => {
                valid_rooms
                    .map(|r| r.id)
                    .sum()
            },
            Part::Two => {
                valid_rooms.map(|r| (r.id, r.dec_name()))
                    .find(|(_, n)| n.contains("north"))
                    .unwrap().0
            }
        }
    }
);

#[derive(Clone, Debug)]
struct Room {
    id: usize,
    checksum: String,
    name: String,
}

impl Room {
    fn check(&self) -> bool {
        let counts = self.name.chars()
            .filter(|&c| c != '-')
            .counts();
        let top_count = counts.iter().sorted_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        // println!("{}[{}]: {top_count:?}", self.id, self.checksum);
        self.checksum.chars()
            .zip(top_count)
            .all(|(a, (&b, _))| a == b)
    }
    fn dec_name(&self) -> String {
        let rot = (self.id % 26) as u8;
        self.name.chars().map(|c| match c {
            '-' => ' ',
            'a'..='z' => {
                let i = (c as u8) - b'a';
                let o = (i + rot) % 26;
                (b'a' + o) as char
            },
            _ => unreachable!("{c} ({})", self.name)
        }).collect()
    }
}
use crate::*;

aoc_day!(
    day = 2,
    output = usize,
    examples = [
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 8),
            (Self::INPUT, 1931),
        ],
        test_cases![
            (Self::EXAMPLES[0], 2286),
            (Self::INPUT, 83105),
        ]
    ],
    solve = |input, part| {
        let games = input.lines()
            .map(Self::parse_line)
            .collect_vec();
        match part {
            Part::One => {
                let constraints = (12,13,14);
                let valid_games: Vec<&Game> = games.iter().filter(|g| {
                    g.rounds
                        .iter()
                        .all(|r| r.0 <= constraints.0 && r.1 <= constraints.1 && r.2 <= constraints.2)
                }).collect();
                //println!("valid games: {:?}", valid_games.iter().map(|g| g.id).collect::<Vec<usize>>());
                valid_games.iter().map(|g| g.id).sum()
            }
            Part::Two => {
                let minimums: Vec<Cubes> = games.iter().map(|g| {
                   Cubes(
                        g.rounds.iter().map(|r| r.0).max().unwrap(),
                        g.rounds.iter().map(|r| r.1).max().unwrap(),
                        g.rounds.iter().map(|r| r.2).max().unwrap()
                   )
                }).collect_vec();
                //println!("minimums: {:?}", minimums);
                minimums.iter().map(|c| c.0 * c.1 * c.2).sum()
            }
        }
    }
);


#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct Game {
    id: usize,
    rounds: Vec<Cubes>,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Cubes(usize, usize, usize); // (r, g, b)

impl Day2 {
    fn parse_line(line: &str) -> Game {
        let (game_text, rounds_text) = line.trim().split_once(':').unwrap();
        let id_text = game_text.trim().split_once(' ').unwrap().1;

        let rounds = rounds_text.split(';')
            .map(Self::parse_round)
            .collect_vec();
        Game {
            id: id_text.parse().unwrap(),
            rounds,
        }
    }

    fn parse_round(round_text: &str) -> Cubes {
        let components = round_text.split(',');
        let mut cubes = Cubes(0, 0, 0);

        for cube_text in components {
            let (count_text, color_text) = cube_text.trim().split_once(' ').unwrap();
            let count = count_text.parse().unwrap();
            match color_text {
                "red" => cubes.0 = count,
                "green" => cubes.1 = count,
                "blue" => cubes.2 = count,
                _ => unreachable!(),
            };
        }
        cubes
    }
}

use crate::*;

aoc_day!(
    day = 10,
    output = usize,
    examples = [],
    tests = [
        test_cases![
            // (Self::EXAMPLES[0], 0),
            (Self::INPUT, 161),
        ],
        test_cases![
            // (Self::EXAMPLES[0], 0),
            (Self::INPUT, 133163),
        ]
    ],
    solve = |input, part| {
        let r = Regex::new(r#"bot (?<bot>\d+) gives low to (?<lo>bot|output) (?<ln>\d+) and high to (?<ho>bot|output) (?<hn>\d+)|\
value (?<val>\d+) goes to bot (?<to>\d+)"#).unwrap();
        let mut bots = FxHashMap::default();
        let mut values = vec![];
        for c in r.captures_iter(input) {
            if let Some(bot) = c.name("bot") {
                bots.insert(bot.usize(), Bot {
                    low: match c.str("lo") {
                        "bot" => Destination::Bot(c.usize("ln")),
                        _ => Destination::Output(c.usize("ln"))
                    },
                    high: match c.str("ho") {
                        "bot" => Destination::Bot(c.usize("hn")),
                        _ => Destination::Output(c.usize("hn"))
                    },
                    holding1: None,
                    holding2: None
                });
            } else {
                values.push((c.usize("to"), c.usize("val")));
            }
        }
        for (b,v) in values {
            let bot = bots.get_mut(&b).unwrap();
            *bot = bot.give(v).unwrap();
        }
        let mut state = State {
            bots,
            bins: FxHashMap::default(),
        };
        match part {
            Part::One => {
                loop {
                    state.step();

                    let Some((i, _)) = state.bots.iter().find(|(_, b)| {
                        matches!((b.holding1, b.holding2), (Some(61), Some(17)) | (Some(17), Some(61)))
                    }) else {continue};
                    return *i;
                }
            },
            Part::Two => {
                loop {
                    state.step();

                    let Some(Some(b0)) = state.bins.get(&0) else {continue};
                    let Some(Some(b1)) = state.bins.get(&1) else {continue};
                    let Some(Some(b2)) = state.bins.get(&2) else {continue};
                    return b0*b1*b2;
                }
            }
        }
    }
);

#[derive(Debug, Clone, PartialEq)]
struct State {
    bots: FxHashMap<usize, Bot>,
    bins: FxHashMap<usize, Option<usize>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Bot {
    low: Destination,
    high: Destination,
    holding1: Option<usize>,
    holding2: Option<usize>,
}

impl Bot {
    fn give(&self, num: usize) -> Option<Self> {
        if self.holding1.is_some() {
            if self.holding2.is_some() {
                None
            } else {
                Some(Bot {
                    low: self.low,
                    high: self.high,
                    holding1: self.holding1,
                    holding2: Some(num),
                })
            }
        } else {
            Some(Bot {
                low: self.low,
                high: self.high,
                holding1: Some(num),
                holding2: None,
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
enum Destination {
    Bot(usize),
    Output(usize),
}

impl State {
    fn step(&mut self) {
            let Some(&i) = self.bots.iter().find(|(_, b)| b.holding1.and(b.holding2).is_some()).map(|(i, _)| i)
                else { panic!("No bot to give!") };

            let bot = self.bots[&i];
            let (a, b) = (bot.holding1.unwrap(), bot.holding2.unwrap());
            let (low, high) = if a < b {(a,b)} else {(b,a)};
            for (num, dest) in [(low, bot.low), (high, bot.high)] {
                match dest {
                    Destination::Output(o) => {
                        self.bins.insert(o, Some(num));
                    },
                    Destination::Bot(b) => {
                        let bot2 = self.bots[&b];
                        self.bots.insert(b, bot2.give(num).unwrap());
                    }
                }
            }
            self.bots.insert(i, Bot {
                low: bot.low,
                high: bot.high,
                holding1: None,
                holding2: None
            });
    }
}
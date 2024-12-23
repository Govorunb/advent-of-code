use num::Integer;
use crate::*;

aoc_day!(
    day = 14,
    output = usize,
    examples = [
"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 1120),
            (Self::INPUT, 2660),
        ],
        test_cases![
            (Self::EXAMPLES[0], 689),
            (Self::INPUT, 1256),
        ]
    ],
    solve = |input, part| {
        // nom? pshhhhhh
        let regex = Regex::new(r#"(?<name>\w+) can fly (?<speed>\d+) km/s for (?<active>\d+) seconds, but then must rest for (?<rest>\d+) seconds."#).unwrap();
        let racers = regex.captures_iter(input)
            .map(|c| {
                Reindeer {
                    name: c.parse("name"),
                    speed: c.parse("speed"),
                    active: c.parse("active"),
                    rest: c.parse("rest")
                }
            }).collect_vec();
        // example/input have different time values, this is for tests
        let time = if racers.len() == 2 {1000} else {2503};
        match part {
            Part::One => {
                racers.iter()
                    .map(|r| r.total_distance(time).0)
                    .max()
                    .unwrap()
            },
            Part::Two => {
                let mut positions = vec![0; racers.len()];
                let mut scores = vec![0usize; racers.len()];
                for t in 0..time {
                    // println!("=== t={} ===", t+1);
                    for (i, r) in racers.iter().enumerate() {
                        // life lesson: never try to be clever
                        positions[i] = r.total_distance(t+1).0;
                        // let time_in_cycle = t % r.cycle_time();
                        // if r.is_moving(t) {
                        //     println!("{} is moving ({}s left)", r.name, r.active - time_in_cycle);
                        //     positions[i] += r.speed;
                        // } else {
                        //     println!("{} is resting ({}s left)", r.name, r.cycle_time() - time_in_cycle);
                        // }
                    }
                    // println!("{positions:?}");
                    let highest = positions.iter().max().unwrap();
                    for (i, _) in positions.iter().enumerate().filter(|&(_i, p)| p == highest) {
                        // println!("{} is leading with {highest}", racers[i].name);
                        scores[i] += 1;
                    }
                    // println!("scores: {scores:?}\n");
                }
                
                *scores.iter().max().unwrap()
            }
        }
    }
);

struct Reindeer {
    name: String,
    speed: usize,
    active: usize,
    rest: usize,
}

impl Reindeer {
    pub fn cycle_time(&self) -> usize {self.active + self.rest}
    pub fn cycle_distance(&self) -> usize {self.speed * self.active}
    
    pub fn total_distance(&self, time: usize) -> (usize, bool) {
        let (cycles, rem) = time.div_rem(&self.cycle_time());
        
        let rem_active = rem.min(self.active);
        
        (cycles * self.cycle_distance() + self.speed * rem_active, rem_active == rem)
    }
    
    pub fn time_for_distance(&self, distance: usize) -> usize {
        let (mut cycles, rem_dist) = distance.div_rem(&self.cycle_distance());
        let mut rem_time = (rem_dist as f64 / self.speed as f64).ceil() as usize;
        // if we divided cleanly, the last cycle has no rest
        if rem_time == 0 {
            rem_time = self.active;
            cycles -= 1;
        }
        
        cycles * self.cycle_time() + rem_time
    }
    
    pub fn is_moving(&self, time: usize) -> bool {
        time % self.cycle_time() <= self.active
    }
}

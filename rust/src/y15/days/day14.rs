use num::Integer;
use crate::*;

pub const DAY14_EXAMPLE: &str =
"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

pub struct Day14 {
    
}

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

impl Day<14> for Day14 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day14.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        // nom? pshhhhhh
        let regex = Regex::new(r#"(?<name>\w+) can fly (?<speed>\d+) km/s for (?<active>\d+) seconds, but then must rest for (?<rest>\d+) seconds."#).unwrap();
        let racers = regex.captures_iter(input)
            .map(|c| {
                let name = c.string("name");
                let speed = c.usize("speed");
                let active = c.usize("active");
                let rest = c.usize("rest");
                Reindeer { name, speed, active, rest }
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
                    for (i, _) in positions.iter().enumerate().filter(|(i, &p)| p == *highest) {
                        // println!("{} is leading with {highest}", racers[i].name);
                        scores[i] += 1;
                    }
                    // println!("scores: {scores:?}\n");
                }
                
                *scores.iter().max().unwrap()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY14_EXAMPLE, 1120),
                (self.input(), 2660),
            ],
            test_cases![
                (DAY14_EXAMPLE, 689),
                (self.input(), 1256),
            ]
        ]
    }
}

impl Default for Day14 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day14 {
    pub fn new() -> Self {
        Self {
        }
    }
}
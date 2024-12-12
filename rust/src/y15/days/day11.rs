use crate::*;

pub struct Day11 {
    
}

impl Day<11> for Day11 {
    type Output = String;
    const INPUT: &'static str = include_str!("../Input/day11.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let starting_pass: &[u8;8] = input.as_bytes().try_into().unwrap();
        
        let mut pass = Vec::from(starting_pass);
        // do while
        loop {
            Self::increment(&mut pass);
            if Self::is_valid(&pass) {break}
        }
        match part {
            Part::One => {
                String::from_utf8(pass).unwrap()
            },
            Part::Two => {
                loop { // again
                    Self::increment(&mut pass);
                    if Self::is_valid(&pass) {break}
                }
                String::from_utf8(pass).unwrap()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                ("abcdefgh", "abcdffaa".to_string()),
                ("ghijklmn", "ghjaabcc".to_string()),
                // (Self::INPUT, 0),
            ],
            test_cases![
                // (Self::INPUT, 0),
            ]
        ]
    }
}

impl Default for Day11 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day11 {
    const A: u8 = 0x61;
    const I: u8 = 0x69;
    const L: u8 = 0x6C;
    const O: u8 = 0x6F;
    const Z: u8 = 0x7A;
    
    pub fn new() -> Self { Self {} }
    
    fn increment(x: &mut [u8]) {
        let mut i = x.len()-1;
        x[i] += 1;
        while x[i] > Self::Z {
            x[i] = Self::A;
            i -= 1;
            x[i] += 1;
        }
    }
    
    fn is_valid(x: &[u8]) -> bool {
        Self::rule1(x) && Self::rule2(x) && Self::rule3(x)
    }
    
    fn rule1(x: &[u8]) -> bool {
        x.iter().tuple_windows().any(|(&a, &b, &c)| b == a+1 && c == a+2)
    }
    
    fn rule2(x: &[u8]) -> bool {
        x.iter().all(|&c| c != Self::I && c != Self::L && c != Self::O)
    }
    fn rule3(x: &[u8]) -> bool {
        let mut doubles = 0;
        let mut last_double: Option<(u8, usize)> = None;
        for i in 0..x.len()-1 {
            let c = x[i];
            if x[i+1] == c && last_double.is_none_or(|(d, pos)| d != c && pos+1 < i) {
                doubles += 1;
                last_double = Some((c, i));
            }
        }
        
        doubles >= 2
    }
}
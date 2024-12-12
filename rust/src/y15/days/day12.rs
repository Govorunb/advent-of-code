use std::ascii;
use crate::*;

pub struct Day12 {
    
}

impl Day<12> for Day12 {
    type Output = isize;
    const INPUT: &'static str = include_str!("../Input/day12.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let regex: Regex = Regex::new(r#"-?\d+"#).unwrap();
        
        match part {
            Part::One => {
                regex.captures_iter(input)
                    .map(|c| c.get(0).unwrap().as_str().parse::<isize>().unwrap())
                    .sum()
            },
            Part::Two => {
                let mut s = input.replace(r#":"red""#, "!");
                while let Some(red) = s.find('!') {
                    let s_ascii = s.as_ascii().unwrap();
                    let mut open = red-1;
                    let mut close = red+1;
                    let mut level = 0;
                    // rfind + ignore nested objects
                    loop {
                        match s_ascii[open] {
                            ascii::Char::LeftCurlyBracket if level == 0 => break,
                            ascii::Char::LeftCurlyBracket => level -= 1,
                            ascii::Char::RightCurlyBracket => level += 1,
                            _ => {},
                        }
                        open -= 1;
                    }
                    // find + ignore nested objects
                    loop {
                        match s_ascii[close] {
                            ascii::Char::RightCurlyBracket if level == 0 => break,
                            ascii::Char::RightCurlyBracket => level -= 1,
                            ascii::Char::LeftCurlyBracket => level += 1,
                            _ => {},
                        }
                        close += 1;
                    }
                    s.replace_range((open+1)..close, ""); // keep the {}
                }

                regex.captures_iter(s.as_str())
                    .map(|c| c.get(0).unwrap().as_str().parse::<isize>().unwrap())
                    .sum()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (r#"[1,2,3]"#, 6),
                (r#"{"a":2,"b":4}"#, 6),
                (r#"[[[3]]]"#, 3),
                (r#"{"a":{"b":4},"c":-1}"#, 3),
                (r#"{"a":[-1,1]}"#, 0),
                (r#"[-1,{"a":1}]"#, 0),
                (r#"[]"#, 0),
                (r#"{}"#, 0),
                (Self::INPUT, 111754),
            ],
            test_cases![
                (r#"[1,2,3]"#, 6),
                (r#"[1,{"c":"red","b":2},3]"#, 4),
                (r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0),
                (r#"[1,"red",5]"#, 6),
                (r#"{"z":999, "nest": {"a":1,"b":2},"c":"red",[{"d":3,"e":4}],"f":5}"#, 0),
                (Self::INPUT, 65402),
            ]
        ]
    }
}

impl Default for Day12 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day12 {
    pub fn new() -> Self {
        Self {
        }
    }
}
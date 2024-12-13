use std::str::FromStr;
use regex::Captures;
use crate::Vector2;
// todo: macro wizardry (if possible)
// accept regex and type
// autofill struct initializer from named captures
// validate regex for named groups - e.g. isize should be -?\d+

pub trait NamedCaptureGroupsHelpers {
    fn str(&self, name: &str) -> &str;
    fn parse<T: FromStr>(&self, name: &str) -> T;
    
    fn string(&self, name: &str) -> String;
    fn usize(&self, name: &str) -> usize;
    fn isize(&self, name: &str) -> isize;
    fn vec2(&self, x_name: &str, y_name: &str) -> Vector2;
}

impl NamedCaptureGroupsHelpers for Captures<'_> {
    fn str(&self, name: &str) -> &str {
        self.name(name).unwrap().as_str()
    }

    fn parse<T: FromStr>(&self, name: &str) -> T {
        let parsed = self.str(name).parse();
        match parsed {
            Ok(parsed) => parsed,
            Err(_) => panic!("Unable to parse {}", name),
        }
    }
    fn string(&self, name: &str) -> String {self.parse(name)}
    fn usize(&self, name: &str) -> usize {self.parse(name)}
    fn isize(&self, name: &str) -> isize {self.parse(name)}
    fn vec2(&self, x_name: &str, y_name: &str) -> Vector2 {
        Vector2 {x: self.parse(x_name), y: self.parse(y_name) }
    }
}

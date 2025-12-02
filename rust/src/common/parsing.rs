use std::str::FromStr;
use regex::{Captures, Match};
use crate::{Size, Vector2};
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
    fn size(&self, x_name: &str, y_name: &str) -> Size;
}

pub trait RegexMatchHelpers {
    fn parse<T: FromStr>(&self) -> T;
    
    fn string(&self) -> String;
    fn usize(&self) -> usize;
    fn isize(&self) -> isize;
}

impl NamedCaptureGroupsHelpers for Captures<'_> {
    fn str(&self, name: &str) -> &str {
        self.name(name).unwrap().as_str()
    }

    fn parse<T: FromStr>(&self, name: &str) -> T {
        self.str(name).parse().unwrap_or_else(|_| panic!("Unable to parse capture {}", name))
    }
    fn string(&self, name: &str) -> String {self.parse(name)}
    fn usize(&self, name: &str) -> usize {self.parse(name)}
    fn isize(&self, name: &str) -> isize {self.parse(name)}
    fn vec2(&self, x_name: &str, y_name: &str) -> Vector2 {
        Vector2 {x: self.parse(x_name), y: self.parse(y_name) }
    }
    fn size(&self, w_name: &str, h_name: &str) -> Size {
        Size {width: self.parse(w_name), height: self.parse(h_name)}
    }
}

impl RegexMatchHelpers for Match<'_> {
    fn parse<T: FromStr>(&self) -> T {self.as_str().parse().unwrap_or_else(|_| panic!("Unable to parse match {}", self.as_str()))}

    fn string(&self) -> String {self.parse()}
    fn usize(&self) -> usize {self.parse()}
    fn isize(&self) -> isize {self.parse()}
}

pub trait IAssureYouItIsMostCertainlyValidASCIIAndAlsoUTF8 {
    fn as_str(&self) -> &str;
}

impl IAssureYouItIsMostCertainlyValidASCIIAndAlsoUTF8 for &[u8] {
    fn as_str(&self) -> &str {
        self.as_ascii().unwrap().as_str()
    }
}

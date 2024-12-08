use regex::Captures;

// todo: macro wizardry (if possible)
// accept regex and type
// autofill struct initializer from the captures
// validate regex for named groups

pub trait NamedCaptureGroupsHelpers {
    fn str(&self, name: &str) -> &str;
    fn string(&self, name: &str) -> String;
    fn usize(&self, name: &str) -> usize;
    fn isize(&self, name: &str) -> isize;
}

impl<'h> NamedCaptureGroupsHelpers for Captures<'h> {
    fn str(&self, name: &str) -> &str {
        self.name(name).unwrap().as_str()
    }
    
    fn string(&self, name: &str) -> String {
        self.str(name).to_string()
    }

    fn usize(&self, name: &str) -> usize {
        self.name(name).unwrap().as_str().parse().unwrap()
    }

    fn isize(&self, name: &str) -> isize {
        self.name(name).unwrap().as_str().parse().unwrap()
    }
}
use crate::{Vector2, Rect};
use std::iter::FusedIterator;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub struct RectIter {
    rect: Rect,
    start: Vector2,
    end: Vector2,
}

impl RectIter {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            start: rect.top_left(),
            end: rect.bottom_right()
        }
    }
}

impl Iterator for RectIter {
    type Item = Vector2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start.y > self.end.y { return None }

        let item = self.start;
        self.start.x += 1;
        if self.start.x > self.rect.bottom_right().x {
            self.start.x = self.rect.top_left().x;
            self.start.y += 1;
        }
        // println!("{:?} forward {:?}", item, self.start);
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl DoubleEndedIterator for RectIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start.y > self.end.y { return None }

        let item = self.end;
        self.end.x -= 1;
        if self.end.x < self.rect.top_left().x {
            self.end.x = self.rect.bottom_right().x;
            self.end.y -= 1;
        }
        // println!("{:?} back {:?}", item, self.end);
        Some(item)
    }
}

impl ExactSizeIterator for RectIter {
    // The default implementation is overly defensive and uses assert_eq! on size_hint
    // we know exactly what we're returning so it's not a problem
    fn len(&self) -> usize {
        match (usize::try_from(self.end.y - self.start.y), usize::try_from(self.end.x - self.start.x)) {
            (Ok(y_diff), Ok(x_diff)) => (self.rect.width() * y_diff) + x_diff,
            _ => 0
        }
    }
}

impl FusedIterator for RectIter {}
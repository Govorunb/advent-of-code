use crate::{Vector2, Rect};
use std::iter::FusedIterator;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub struct RectIter {
    start: Vector2,
    end: Vector2,
    front: Vector2,
    back: Vector2,
}

impl RectIter {
    pub fn new(rect: Rect) -> Self {
        let start = rect.top_left();
        let end = rect.bottom_right();
        
        Self {
            start,
            end,
            front: start,
            back: end,
        }
    }
}

impl Iterator for RectIter {
    type Item = Vector2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front.y > self.back.y { return None }

        let item = self.front;
        self.front.x += 1;
        if self.front.x > self.end.x {
            self.front.x = self.start.x;
            self.front.y += 1;
        }
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl DoubleEndedIterator for RectIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back.y < self.start.y { return None }

        let item = self.back;
        self.back.x -= 1;
        if self.back.x < self.start.x {
            self.back.x = self.end.x;
            self.back.y -= 1;
        }
        // println!("{:?} back {:?}", item, self.back);
        Some(item)
    }
}

impl ExactSizeIterator for RectIter {
    // The default implementation is overly defensive and uses assert_eq! on size_hint
    // we know exactly what we're returning so it's not a problem
    fn len(&self) -> usize {
        let width = 1 + self.end.x - self.start.x;
        if self.back.y < self.front.y {
            0
        } else {
            let y_rem = self.back.y - self.front.y;
            let x_rem = 1 + self.back.x - self.front.x;
            (width * y_rem + x_rem) as usize
        }
    }
}

impl FusedIterator for RectIter {}

#[cfg(test)]
mod test {
    #[test]
    fn test_rect_iter() {
        use crate::{Rect, Size, Vector2};
        
        for width in 3..10 {
            for height in 3..10 {
                let rect = Rect::from_origin(Size {width, height}).unwrap();
                let mut iter = rect.into_iter();
                let full_len = width * height;
                assert_eq!(iter.size_hint(), (full_len, Some(full_len)));
                assert_eq!(iter.next(), Some(Vector2{x: 0, y: 0}));
                assert_eq!(iter.size_hint(), (full_len-1, Some(full_len-1)));
                iter.nth(full_len - 3); // consume all but last element
                assert_eq!(iter.size_hint(), (1, Some(1)));
                assert_eq!(iter.next(), Some((width-1, height-1).into()));
                assert_eq!(iter.size_hint(), (0, Some(0)));
            }
        }
    }
}

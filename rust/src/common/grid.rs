use std::{fmt::Display, hash::Hash, ops::{Index, IndexMut}, slice::{self, ChunksExact, ChunksExactMut}, str::FromStr};

use crate::*;
use itertools::*;
use num::Integer;
use strided::*;

/// An opinionated mishmash reimplementation of [`Array2D`] and [`Vec2D`].
/// The differences are:
/// - renamed/refactored methods that return [Iterator]s to suit my conventions
/// - implemented [FromStr]/[Display] for element types that implement [From<char>]/[Display] themselves
/// 
/// [`Array2D`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html
/// [`Vec2D`]: https://docs.rs/vec2d/latest/vec2d/struct.Vec2D.html
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    rect: Rect,
    elements: Vec<T>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Error {
    /// Not all rows/columns have the same length
    DimensionsNotUniform,
    /// Provided dimensions don't match the data - either not enough or too many elements
    IncorrectDimensions,
    /// Tried to index into the grid with out-of-bounds coordinates
    CoordsOutOfBounds(usize, usize),
    /// Tried to index into a row or column with an out-of-bounds index
    IndexOutOfBounds(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub enum Major {
    #[default]
    Row,
    Column,
}

impl From<(usize, usize)> for Size {
    fn from(tuple: (usize, usize)) -> Self {
        Self { width: tuple.0, height: tuple.1 }
    }
}

impl<TErr, T: TryFrom<char, Error = TErr> + Clone> FromStr for Grid<T> {
    type Err = Either<TErr, Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let total = s.len();
        let width = s.lines().next().unwrap().len();
        let mut elements = Vec::with_capacity(total);
        for line in s.lines() {
            if line.len() != width {
                return Err(Either::Right(Error::DimensionsNotUniform));
            }
            let row: Vec<T> = line.chars()
                .map(|c| c.try_into().map_err(|e| Either::Left(e)))
                .try_collect()?;
            elements.extend(row);
        }
        elements.shrink_to_fit();
        let (height, rem) = elements.len().div_rem(&width);
        if rem != 0 {
            return Err(Either::Right(Error::DimensionsNotUniform));
        }
        
        Ok(Self {
            elements,
            rect: Rect::from_origin(Size { width, height }).unwrap()
        })
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize { self.rect.width() }
    pub fn height(&self) -> usize { self.rect.height() }
    pub fn size(&self) -> Size { self.rect.size() }
    pub fn base(&self) -> Point { self.rect.top_left() }
    
    /// Returns the flattened index that corresponds to the given coordinates.
    /// "flattened" here means the index in a (totally hypothetical) backing 1D array
    pub fn flat_index(&self, x: isize, y: isize) -> Option<usize> {
        let x: usize = (x - self.base().x).try_into().ok()?;
        let y: usize = (y - self.base().y).try_into().ok()?;
        if x >= self.width() || y >= self.height() {
            return None;
        }
        let flat = y * self.width() + x;
        if flat < self.elements.len() {
            Some(flat)
        } else {
            None
        }
    }
    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        self.flat_index(x, y).map(|i| &self.elements[i])
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        self.flat_index(x, y).map(|i| &mut self.elements[i])
    }
    
    /// Returns the element of the grid at the given flattened index, in row-major order.
    pub fn flat_element(&self, i: usize) -> Option<&T> {
        self.elements.get(i)
    }

    /// Returns the element of the grid at the given flattened index, in column-major order.
    pub fn transposed_flat_element(&self, i: usize) -> Option<&T> {
        if i >= self.width() * self.height() {
            return None;
        }
        let (x, y) = i.div_rem(&self.height());
        self.get(x as isize, y as isize)
    }

    pub fn flat_element_mut(&mut self, i: usize) -> Option<&mut T> {
        self.elements.get_mut(i)
    }
    
    pub fn rows(&self) -> ChunksExact<T> {
        self.elements.chunks_exact(self.width())
    }

    pub fn rows_mut(&mut self) -> ChunksExactMut<T> {
        self.elements.chunks_exact_mut(self.rect.width())
    }

    pub fn row(&self, row: usize) -> Option<&[T]> {
        self.flat_index(self.base().x, self.base().y + row as isize)
            .map(|i| &self.elements[i..i+self.width()])
    }

    pub fn row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        self.flat_index(self.base().x, self.base().y + row as isize)
            .map(|i| &mut self.elements[i..i+self.rect.width()])
    }

    pub fn col(&self, x: usize) -> Option<Stride<T>> {
        self.cols().nth(x)
    }

    pub fn col_mut(&mut self, x: usize) -> Option<MutStride<T>> {
        self.cols_mut().nth(x)
    }

    pub fn cols(&self) -> Substrides<T> {
        self.elements
            .as_stride()
            .substrides(self.width())
    }

    pub fn cols_mut(&mut self) -> MutSubstrides<T> {
        self.elements
            .as_stride_mut()
            .substrides_mut(self.rect.width())
    }

    pub fn elements(&self) -> slice::Iter<T> {
        self.elements.iter()
    }

    pub fn elements_mut(&mut self) -> slice::IterMut<T> {
        self.elements.iter_mut()
    }

    pub fn coords(&self) -> RectIter {
        RectIter::new(self.rect.clone())
    }

    pub fn cells(&self) -> impl DoubleEndedIterator<Item = (isize, isize, &T)> + Clone {
        self.coords()
            .map(move |pt| (pt.x, pt.y, &self[pt]))
    }

    pub fn cells_mut(&mut self) -> impl DoubleEndedIterator<Item = (usize, usize, &mut T)> {
        // can't do it the other way
        // (`self.coords().map(|(x,y)| &mut self[(x,y)])`)
        // because closure can't capture self
        // and i cba to make an iterator struct
        let w = self.width();
        self.elements_mut()
            .enumerate() // can't zip because zip isn't double ended
            .map(move |(i, e)| (i % w, i / w, e))
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(rect: Rect) -> Self {
        Self {
            elements: vec![T::default(); rect.width() * rect.height()],
            rect,
        }
    }
    
    pub fn from_origin(size: Size) -> Option<Self> {
        let rect = Rect::from_origin(size)?;
        Some(Self::new(rect))
    }

    pub fn clear(&mut self) {
        self.elements.fill(T::default());
    }
}

impl<T: Clone> Grid<T> {
    pub fn fill_with(size: Size, elem: T) -> Option<Self> {
        let rect = Rect::from_origin(size)?;
        Some(Self {
            rect,
            elements: vec![elem; size.width * size.height]
        })
    }

    pub fn to_vec_column_major(&self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.width() * self.height());
        
        for y in self.rect.y_range() {
            for x in self.rect.x_range() {
                vec.push(self[Point::from((x, y))].clone());
            }
        }
        vec
    }
    pub fn as_vec_row_major(&self) -> &Vec<T> {
        &self.elements
    }

    pub fn to_rows(&self) -> Vec<Vec<T>> {
        self.rows()
            .map(|r| r.to_vec())
            .collect_vec()
    }

    pub fn to_cols(&self) -> Vec<Vec<T>> {
        self.cols()
            .map(|c| Vec::from_iter(c.iter().cloned()))
            .collect_vec()
    }
}

// no implementation for IntoIterator for Grid<T>
// because it would require a lifetime

impl<'a, T: 'a> IntoIterator for &'a Grid<T> {
    type Item = &'a [T];
    type IntoIter = ChunksExact<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows()
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.elements[i]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.elements[i]
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, pt: Point) -> &Self::Output {
        self.get(pt.x, pt.y)
            .unwrap_or_else(|| panic!("Index coordinates ({}, {}) out of bounds", pt.x, pt.y))
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, pt: Point) -> &mut Self::Output {
        self.get_mut(pt.x, pt.y)
            .unwrap_or_else(|| panic!("Index mut coordinates ({}, {}) out of bounds", pt.x, pt.y))
    }
}

impl<T: Sync + Send> Grid<T> {
    pub fn par_elements(&self) -> rayon::slice::Iter<'_, T> {
        self.elements.par_iter()
    }
    pub fn par_elements_mut(&mut self) -> rayon::slice::IterMut<'_, T> {
        self.elements.par_iter_mut()
    }
    pub fn par_rows(&self) -> rayon::slice::ChunksExact<'_, T> {
        self.elements.par_chunks_exact(self.rect.width())
    }
    pub fn par_rows_mut(&mut self) -> rayon::slice::ChunksExactMut<'_, T> {
        self.elements.par_chunks_exact_mut(self.rect.width())
    }
}
use std::{fmt::Display, hash::Hash, iter::FusedIterator, ops::{Index, IndexMut}, slice::{self, ChunksExact, ChunksExactMut}, str::FromStr};

use super::*;
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
    size: Size,
    elements: Vec<T>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
            size: Size { width, height }
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
    pub fn width(&self) -> usize { self.size.width }
    pub fn height(&self) -> usize { self.size.height }
    pub fn size(&self) -> Size { self.size }
    
    /// Returns the flattened index that corresponds to the given coordinates.
    /// "flattened" here means the index in a (totally hypothetical) backing 1D array
    pub fn flat_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width() && y < self.height() {
            Some(y * self.width() + x)
        } else {
            None
        }
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.flat_index(x, y).map(|i| &self.elements[i])
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
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
        self.get(x, y)
    }

    pub fn flat_element_mut(&mut self, i: usize) -> Option<&mut T> {
        self.elements.get_mut(i)
    }

    pub fn rows(&self) -> ChunksExact<T> {
        self.elements.chunks_exact(self.size.width)
    }

    pub fn rows_mut(&mut self) -> ChunksExactMut<T> {
        self.elements.chunks_exact_mut(self.size.width)
    }

    pub fn row(&self, y: usize) -> Option<&[T]> {
        self.flat_index(0, y).map(|i| &self.elements[i..i+self.size.width])
    }

    pub fn row_mut(&mut self, y: usize) -> Option<&mut [T]> {
        self.flat_index(0, y).map(|i| &mut self.elements[i..i+self.size.width])
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
            .substrides(self.size.width)
    }

    pub fn cols_mut(&mut self) -> MutSubstrides<T> {
        self.elements
            .as_stride_mut()
            .substrides_mut(self.size.width)
    }

    pub fn elements(&self) -> slice::Iter<T> {
        self.elements.iter()
    }

    pub fn elements_mut(&mut self) -> slice::IterMut<T> {
        self.elements.iter_mut()
    }

    pub fn coords(&self) -> CoordsIter {
        CoordsIter::new(self)
    }

    pub fn cells(&self) -> impl DoubleEndedIterator<Item = (usize, usize, &T)> + Clone {
        self.coords()
            .map(move |(x, y)| (x, y, &self[(x,y)]))
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
    pub fn new(size: Size) -> Self {
        Self {
            size,
            elements: vec![T::default(); size.width * size.height]
        }
    }

    pub fn clear(&mut self) {
        self.elements.fill(T::default());
    }
}

impl<T: Clone> Grid<T> {
    pub fn fill_with(&self, size: Size, elem: T) -> Self {
        Self {
            size,
            elements: vec![elem; size.width * size.height]
        }
    }

    pub fn to_vec_column_major(&self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.width() * self.height());
        for y in 0..self.height() {
            for x in 0..self.width() {
                vec.push(self[(x,y)].clone());
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

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.get(x, y)
            .unwrap_or_else(|| panic!("Index coordinates ({}, {}) out of bounds", x, y))
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.get_mut(x, y)
            .unwrap_or_else(|| panic!("Index mut coordinates ({}, {}) out of bounds", x, y))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CoordsIter {
    stride: usize,
    start: usize,
    end: usize,
}

impl CoordsIter {
    pub fn new<T>(grid: &Grid<T>) -> Self {
        let total = grid.width() * grid.height();
        Self {
            stride: grid.width(),
            start: 0,
            end: total,
        }
    }

    fn coords(&self, i: usize) -> (usize, usize) {
        let (y,x) = i.div_rem(&self.stride);
        (x,y)
    }
}

impl Iterator for CoordsIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end { return None }

        let item = self.coords(self.start);
        self.start += 1;
        Some(item)
    }

    // never should've put this on Iterator in the first place
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl DoubleEndedIterator for CoordsIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start >= self.end { return None }

        self.end -= 1;
        Some(self.coords(self.end))
    }
}

impl ExactSizeIterator for CoordsIter {
    // The default implementation is overly defensive and uses assert_eq! on size_hint
    // we know exactly what we're returning so it's not a problem
    fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

impl FusedIterator for CoordsIter {}

impl<T: Sync + Send> Grid<T> {

    pub fn par_elements(&self) -> rayon::slice::Iter<'_, T> {
        self.elements.par_iter()
    }
    pub fn par_elements_mut(&mut self) -> rayon::slice::IterMut<'_, T> {
        self.elements.par_iter_mut()
    }
    pub fn par_rows(&self) -> rayon::slice::ChunksExact<'_, T> {
        self.elements.par_chunks_exact(self.size.width)
    }
    pub fn par_rows_mut(&mut self) -> rayon::slice::ChunksExactMut<'_, T> {
        self.elements.par_chunks_exact_mut(self.size.width)
    }
}
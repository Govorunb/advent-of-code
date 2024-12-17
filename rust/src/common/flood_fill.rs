use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};
use crate::*;

/// ## Arguments
/// - `start` - the starting item to initiate the flood fill from.
/// - `neighbours` - function that generates which tiles to examine next, based on the current tile
/// - `filter` - a [`Fn`] (current_tile: Vector2, adjacent_tile: Vector2) -> bool
///     - determines whether to queue `adjacent_tile` to examine its neighbours
///     - most common checks include checking if a position is inside a grid, or if the element at that position has some specific property
pub fn flood_fill<T, G, I, F>(start: &T, neighbours: G, mut filter: F) -> impl Iterator<Item = T>
where
    T: Eq + Clone + Hash,
    G: Fn(&T) -> I,
    I: Iterator<Item = T>,
    F: FnMut(&T, &T) -> bool,
{
    std::iter::from_coroutine(#[coroutine] move || {
        let mut to_examine: FxHashSet<T> = FxHashSet::from_iter([start.clone()]);
        let mut results: FxHashSet<T> = FxHashSet::default();
        while let Some(tile) = pop_from_set(&mut to_examine) {
            if results.insert(tile.clone()) {
                yield tile.clone();
            }
            for adj in neighbours(&tile) {
                if !results.contains(&adj) && !to_examine.contains(&adj)
                    && filter(&tile, &adj)
                {
                    to_examine.insert(adj);
                }
            }
        }
    })
}

fn pop_from_set<T,H>(set: &mut HashSet<T, H>) -> Option<T>
where 
    T: Eq + Clone + Hash,
    H: BuildHasher
{
    set.iter().next().cloned()
        .and_then(|item| set.take(&item))
}

pub fn flood_fill_adjacent<P>(start: &Vector2, filter: P) -> impl Iterator<Item = Vector2>
where
    P: FnMut(&Vector2, &Vector2) -> bool,
{
    flood_fill(start, |pt| pt.adjacent(), filter)
}

pub fn flood_fill_around<P>(start: &Vector2, filter: P) -> impl Iterator<Item = Vector2>
where
    P: FnMut(&Vector2, &Vector2) -> bool,
{
    flood_fill(start, |pt| pt.around(), filter)
}

use crate::*;

/// ## Arguments
/// - `start` - the starting item to initiate the flood fill from.
/// - `neighbours` - function that generates which tiles to examine next, based on the current tile
/// - `filter` - a [`Fn`] (current_tile: Vector2, adjacent_tile: Vector2) -> bool
///     - determines whether to queue `adjacent_tile` to examine its neighbours
///     - most common checks include checking if a position is inside a grid, or if the element at that position has some specific property
pub fn flood_fill<T, G, I, F>(start: &T, neighbours: G, mut filter: F) -> Vec<T>
where
    T: PartialEq + Clone,
    G: Fn(&T) -> I,
    I: Iterator<Item = T>,
    F: FnMut(&T, &T) -> bool,
{
    let mut white: Vec<T> = vec![start.clone()];
    let mut black: Vec<T> = vec![];
    while let Some(tile) = white.pop() {
        if !black.contains(&tile) {
            black.push(tile.clone());
        }
        for adj in neighbours(&tile) {
            if !black.contains(&adj) && !white.contains(&adj)
                && filter(&tile, &adj)
            {
                white.push(adj);
            }
        }
    }
    
    black
}

pub fn flood_fill_adjacent<P>(start: &Vector2, filter: P) -> Vec<Vector2>
where
    P: FnMut(&Vector2, &Vector2) -> bool,
{
    flood_fill(start, |pt| pt.adjacent(), filter)
}

pub fn flood_fill_around<P>(start: &Vector2, filter: P) -> Vec<Vector2>
where
    P: FnMut(&Vector2, &Vector2) -> bool,
{
    flood_fill(start, |pt| pt.around(), filter)
}
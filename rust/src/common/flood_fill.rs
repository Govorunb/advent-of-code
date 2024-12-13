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
    // stack (Vec<T>, push/pop) makes DFS, queue (VecDeque<T>, push_back/pop_front) makes BFS
    // BFS may be more appropriate in some scenarios; DFS, on the other hand, generally doesn't explode in memory usage
    let mut to_examine: Vec<T> = vec![start.clone()];
    // maybe make this a generator/iterator?
    let mut results: Vec<T> = vec![];
    while let Some(tile) = to_examine.pop() {
        if !results.contains(&tile) {
            results.push(tile.clone());
        }
        for adj in neighbours(&tile) {
            if !results.contains(&adj) && !to_examine.contains(&adj)
                && filter(&tile, &adj)
            {
                to_examine.push(adj);
            }
        }
    }

    results
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

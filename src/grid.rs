//! Datastructure allowing for classification and indexing of data by location in a grid.
//!
//! # Grid
//!
//! Contains the definition and methods used to define a hybrid datastructure
//! consisting of a combination of a 2D array and a linked list.
//!
//! ## Function
//! This datastructure aims to allow for a quicker indexing of values based on
//! their location in a 2D grid. By roughly knowing the location of the target value
//! in this grid, only a subset of nearby values need to be checked.
//! This happens by both storing all values in a combined vector and having these
//! values link to other values in a given cell of the grid. Then by finding the
//! first value in a cell, all other values in the cell can be discovered.
//!
//! ## Iterators
//! To allow for easier use of this datastructure, two iterators are
//! given. [`IndexIter<T>`] can be created using the [`Grid::iter_from_index`]
//! and [`Grid::iter_from_pos`] methods, which return an iterator over all indices of
//! values in a given cell.
//! [`Iter<T>`] can be created using the [`Grid::iter_all`] method, which returns
//! an iterator over all values in the grid, independent of which cell these values are in.

// Index given to any non-existing value, similar to c's NULL.
const EMPTY: i32 = -1;

/// Stores the value in the [`Grid<T>`], along with the index of the next value in the
/// list of all values.
pub struct ValueNode<T> {
    pub next_index: i32,
    pub val: T,
}

/// Value at each cell of the [`Grid<T>`].
/// This value contains the `first` and `last` index of the value in the cell,
/// along with the total `count` of values in the cell.
#[derive(Clone, Copy)]
pub struct GridNode {
    pub first: i32,
    pub last: i32,
    pub count: u32,
}

/// Defines the custom grid-like linked list datastructure.
/// The values are stored in `values`, while the starting indices
/// of the linked lists are located in the `grid` field.
pub struct Grid<T> {
    /// The values in the grid, contained in [`ValueNode`]s
    /// to link to the next value in a cell.
    pub values: Vec<ValueNode<T>>,
    /// 2D grid mapped to a vec with the starting indices of the linked lists.
    pub grid: Vec<GridNode>,
    /// The total amount of values stored in the grid.
    pub count: usize,
    /// The amount of rows in the grid.
    pub rows: usize,
    /// The amount of columns in the grid.
    pub columns: usize,
}

impl<'a, T> Grid<T> {
    /// The index representing an empty cell, or the end of a linked list.
    pub const EMPTY: i32 = EMPTY;

    /// Creates a new [`Grid<T>`].
    pub fn new(max_count: usize, columns: usize, rows: usize) -> Grid<T> {
        let grid = vec![
            GridNode {
                first: Self::EMPTY,
                last: Self::EMPTY,
                count: 0
            };
            columns * rows
        ];
        Grid {
            values: Vec::with_capacity(max_count),
            grid,
            count: 0,
            columns,
            rows,
        }
    }

    /// Returns the iterator over all of the values in the [`Grid<T>`].
    pub fn iter_all(&'a self) -> Iter<'a, T> {
        Iter::new(self)
    }

    /// Creates an [`IndexIter`] object, which iteratively returns indices of
    /// elements linked in the cell given by `column` and `row`.
    #[allow(dead_code)]
    #[inline]
    pub fn iter_from_pos(&'a self, column: i32, row: i32) -> IndexIter<'a, T> {
        let value_index = if let Some(grid_node) = self.get_grid_node(row, column) {
            grid_node.first
        } else {
            Self::EMPTY
        };
        IndexIter::new(value_index, self)
    }

    /// See [`Grid::iter_from_pos`].
    #[allow(dead_code)]
    #[inline]
    pub fn iter_from_index(&'a self, cell_index: i32) -> IndexIter<'a, T> {
        if cell_index < 0 {
            IndexIter::new(Self::EMPTY, self)
        } else {
            IndexIter::new(self.grid[cell_index as usize].first, self)
        }
    }

    /// Returns the index of the cell given by `row` and `column`, or
    /// [`Grid::EMPTY`] if the position falls outside of the grid.
    #[inline]
    pub fn index_from_pos(&self, row: i32, column: i32) -> i32 {
        if row >= 0 && (row as usize) < self.rows && column >= 0 && (column as usize) < self.columns
        {
            column + row * self.columns as i32
        } else {
            Self::EMPTY
        }
    }

    /// Get the value at the given `index` in the value vec.
    ///
    /// # Errors
    ///
    /// This function will return an error if the index falls outside of the values vec.
    #[inline]
    pub fn get_val(&self, index: usize) -> Result<&T, ()> {
        if index < self.count {
            Ok(&self.values[index].val)
        } else {
            Err(())
        }
    }

    /// Get the [`GridNode`] at the location given by `row` and `column`, or
    /// `None` if the location falls outside of the grid.
    pub fn get_grid_node(&self, row: i32, column: i32) -> Option<GridNode> {
        let grid_index = self.index_from_pos(row, column);
        if grid_index != Self::EMPTY {
            Some(self.grid[grid_index as usize])
        } else {
            None
        }
    }

    /// Add a new `val` to the grid at a cell given by `row` and `column`.
    /// If the given location does not fall in the grid, the value is only
    /// added to the values vec and not to any cell.
    pub fn add_val(&mut self, val: T, row: i32, column: i32) {
        let mut next_index = -1;
        let grid_index = self.index_from_pos(row, column);
        if grid_index != Self::EMPTY {
            let grid_index = grid_index as usize;
            next_index = self.grid[grid_index].first;
            self.grid[grid_index].first = self.count as i32;
            self.grid[grid_index].count += 1;
            // If the cell was empty.
            if next_index == Self::EMPTY {
                self.grid[grid_index].last = self.count as i32;
            }
        }
        let node = ValueNode { val, next_index };
        self.values.push(node);
        self.count += 1;
    }

    /// Unlinks a value node with the given `index` from its [`GridNode`] at
    /// `grid_row` and `grid_column` in the grid.
    /// This function requires the user to manually find the index of the
    /// previous node in the cell, giving a negative value for `prev_index`
    /// if there is none.
    pub fn unlink_val(&mut self, index: usize, prev_index: i32, grid_row: i32, grid_column: i32) {
        let grid_index = self.index_from_pos(grid_row, grid_column);
        if grid_index >= 0 {
            let next_index = self.values[index].next_index;
            let grid_index = grid_index as usize;
            let grid_node = &mut self.grid[grid_index];
            // Current boid is first
            if prev_index == self::EMPTY {
                if grid_node.first != index as i32 {
                    panic!("Incorrect previous index for value.");
                }
                grid_node.first = next_index;
            } else {
                // Other boids before in grid.
                let prev_node = &mut self.values[prev_index as usize];
                if prev_node.next_index != index as i32 {
                    panic!("Incorrect previous index for value.");
                }
                prev_node.next_index = next_index;
            }

            if grid_node.last == index as i32 {
                grid_node.last = prev_index;
            }
            grid_node.count -= 1;
        }
    }

    /// Links an exististing value that is currently not in a cell to the end of
    /// a cell given by `grid_row` and `grid_column`.
    pub fn link_val(&mut self, index: usize, grid_row: i32, grid_column: i32) {
        let grid_index = self.index_from_pos(grid_row, grid_column);
        // If the position falls in the grid.
        if grid_index != self::EMPTY {
            let grid_index = grid_index as usize;
            self.values[index].next_index = -1;
            let grid_node = &mut self.grid[grid_index];
            let last_index = grid_node.last;
            if last_index >= 0 {
                self.values[last_index as usize].next_index = index as i32;
            } else {
                grid_node.first = index as i32;
            }
            grid_node.last = index as i32;
            grid_node.count += 1;
        }
    }
}

/// Iterator over the indices of values in given cell in the [`Grid<T>`].
pub struct IndexIter<'a, T: 'a> {
    current: i32,
    values: &'a Vec<ValueNode<T>>,
}

impl<'a, T> IndexIter<'a, T> {
    /// Creates a new [`IndexIter<T>`].
    fn new(value_index: i32, grid: &'a Grid<T>) -> IndexIter<'a, T> {
        IndexIter {
            current: value_index,
            values: &grid.values,
        }
    }
}

impl<'a, T: 'a> Iterator for IndexIter<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == EMPTY {
            return None;
        }
        // Traverse the cell's linked list.
        let curr_node = &self.values[self.current as usize];
        let current = self.current;
        self.current = curr_node.next_index;
        Some(current as usize)
    }
}

/// Iterator over all values in the [`Grid<T>`].
pub struct Iter<'a, T: 'a> {
    current: std::slice::Iter<'a, ValueNode<T>>,
}

impl<'a, T> Iter<'a, T> {
    /// Creates a new [`Iter<T>`].
    fn new(grid: &'a Grid<T>) -> Iter<'a, T> {
        Iter {
            current: grid.values.iter(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.next() {
            Some(value) => Some(&value.val),
            None => None,
        }
    }
}

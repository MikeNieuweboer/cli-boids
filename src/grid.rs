const NULL: i32 = -1;

pub struct ValueNode<T> {
    pub val: T,
    pub next_index: i32,
}

#[derive(Clone, Copy)]
pub struct GridNode {
    pub first: i32,
    pub last: i32,
    pub count: u32,
}

pub struct Grid<T> {
    pub values: Vec<ValueNode<T>>,
    pub grid: Vec<GridNode>,
    pub count: usize,
    pub rows: usize,
    pub columns: usize,
}

impl<'a, T> Grid<T> {
    pub fn new(max_count: usize, columns: usize, rows: usize) -> Grid<T> {
        let grid = vec![
            GridNode {
                first: NULL,
                last: NULL,
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

    #[allow(dead_code)]
    #[inline]
    pub fn iter(&'a self, column: i32, row: i32) -> IndexIter<'a, T> {
        let value_index = if let Some(grid_node) = self.get_grid_node(row, column) {
            grid_node.first
        } else {
            NULL
        };
        IndexIter::new(value_index, self)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn iter_from_index(&'a self, cell_index: i32) -> IndexIter<'a, T> {
        if cell_index < 0 {
            IndexIter::new(NULL, self)
        } else {
            IndexIter::new(self.grid[cell_index as usize].first, self)
        }
    }

    #[inline]
    pub fn index_from_pos(&self, row: i32, column: i32) -> i32 {
        if row >= 0 && (row as usize) < self.rows && column >= 0 && (column as usize) < self.columns
        {
            column + row * self.columns as i32
        } else {
            NULL
        }
    }

    #[inline]
    pub fn get_val(&self, index: usize) -> &T {
        &self.values[index].val
    }

    pub fn get_grid_node(&self, row: i32, column: i32) -> Option<GridNode> {
        let grid_index = self.index_from_pos(row, column);
        if grid_index != NULL {
            Some(self.grid[grid_index as usize])
        } else {
            None
        }
    }

    pub fn add_val(&mut self, val: T, row: i32, column: i32) {
        let mut next_index = -1;
        let grid_index = self.index_from_pos(row, column);
        if grid_index != NULL {
            let grid_index = grid_index as usize;
            next_index = self.grid[grid_index].first;
            self.grid[grid_index].first = self.count as i32;
            self.grid[grid_index].count += 1;
            // If the cell was empty.
            if next_index == NULL {
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
            if prev_index == -1 {
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

    pub fn link_val(&mut self, index: usize, grid_row: i32, grid_column: i32) {
        let grid_index = self.index_from_pos(grid_row, grid_column);
        if grid_index >= 0 {
            self.values[index].next_index = -1;
            let grid_index = grid_index as usize;
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

pub struct IndexIter<'a, T: 'a> {
    current: i32,
    values: &'a Vec<ValueNode<T>>,
}

impl<'a, T> IndexIter<'a, T> {
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
        if self.current == NULL {
            return None;
        }
        let curr_node = &self.values[self.current as usize];
        let current = self.current;
        self.current = curr_node.next_index;
        Some(current as usize)
    }
}

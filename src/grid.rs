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

pub struct Iter<'a, T: 'a> {
    current: i32,
    grid: &'a Grid<T>,
}

impl<'a, T> Iter<'a, T> {
    fn new(value_index: i32, grid: &'a Grid<T>) -> Iter<'a, T> {
        Iter {
            current: value_index,
            grid,
        }
    }
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == NULL {
            return None;
        }
        let curr_node = &self.grid.values[self.current as usize];
        let current = self.current;
        self.current = curr_node.next_index;
        Some(current as usize)
    }
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
    pub fn iter(&'a self, column: i32, row: i32) -> Iter<'a, T> {
        let value_index = if let Some(grid_node) = self.get_grid_node(row, column) {
            grid_node.first
        } else {
            NULL
        };
        Iter::new(value_index, self)
    }

    #[allow(dead_code)]
    pub fn iter_from_index(&'a self, cell_index: i32) -> Iter<'a, T> {
        if cell_index < 0 {
            Iter::new(NULL, self)
        } else {
            Iter::new(self.grid[cell_index as usize].first, self)
        }
    }

    pub fn index_from_pos(&self, row: i32, column: i32) -> i32 {
        if row >= 0 && (row as usize) < self.rows && column >= 0 && (column as usize) < self.columns
        {
            column + row * self.columns as i32
        } else {
            NULL
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
}

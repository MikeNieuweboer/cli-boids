pub struct ValueNode<T> {
    pub val: T,
    pub next_index: i32,
}

#[derive(Clone, Copy)]
pub struct GridNode {
    pub index: i32,
    pub count: u32,
}

pub struct Grid<T> {
    pub values: Vec<ValueNode<T>>,
    pub grid: Vec<GridNode>,
    pub count: usize,
    pub rows: usize,
    pub columns: usize,
}

impl<T> Grid<T> {
    pub fn new(max_count: usize, columns: usize, rows: usize) -> Grid<T> {
        let grid = vec![
            GridNode {
                index: -1,
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

    pub fn add_val(&mut self, val: T, column: i32, row: i32) {
        let mut next_index = -1;
        if row >= 0 && (row as usize) < self.rows && column >= 0 && (column as usize) < self.columns
        {
            let grid_index = column as usize + row as usize * self.columns;
            next_index = self.grid[grid_index].index;
            self.grid[grid_index].index = self.count as i32;
            self.grid[grid_index].count += 1;
        }
        let node = ValueNode { val, next_index };
        self.values.push(node);
        self.count += 1;
    }

    pub fn get_val_index(&self, column: usize, row: usize) -> i32 {
        self.grid[column + row * self.columns].index
    }

    pub fn get_grid_node(&self, column: usize, row: usize) -> GridNode {
        self.grid[column + row * self.columns]
    }
}

use super::cell::{Cell, CellValue};
use std::collections::HashMap;

/// Main grid data structure optimized for sparse data
pub struct Grid {
    rows: usize,
    cols: usize,
    // Sparse storage: only store non-empty cells
    // Key: (row, col), Value: Cell
    cells: HashMap<(usize, usize), Cell>,

    // Column widths (in pixels)
    col_widths: Vec<f32>,

    // Row heights (in pixels)
    row_heights: Vec<f32>,

    // Default dimensions
    default_col_width: f32,
    default_row_height: f32,

    // Header dimensions
    pub row_header_width: f32,
    pub col_header_height: f32,
    pub show_headers: bool,
}

impl Grid {
    /// Create a new grid with specified dimensions
    pub fn new(rows: usize, cols: usize) -> Self {
        let default_col_width = 100.0;
        let default_row_height = 25.0;

        Self {
            rows,
            cols,
            cells: HashMap::new(),
            col_widths: vec![default_col_width; cols],
            row_heights: vec![default_row_height; rows],
            default_col_width,
            default_row_height,
            row_header_width: 60.0,
            col_header_height: 30.0,
            show_headers: true,
        }
    }

    /// Get number of rows
    pub fn row_count(&self) -> usize {
        self.rows
    }

    /// Get number of columns
    pub fn col_count(&self) -> usize {
        self.cols
    }

    /// Get cell at position (row, col)
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(&(row, col))
    }

    /// Get mutable cell at position (row, col)
    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.cells.get_mut(&(row, col))
    }

    /// Set cell at position (row, col)
    pub fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        if row < self.rows && col < self.cols {
            self.cells.insert((row, col), cell);
        }
    }

    /// Set cell value at position (row, col)
    pub fn set_value(&mut self, row: usize, col: usize, value: CellValue) {
        if row < self.rows && col < self.cols {
            self.cells
                .entry((row, col))
                .or_insert_with(Cell::default)
                .value = value;
        }
    }

    /// Get cell value as string
    pub fn get_value_string(&self, row: usize, col: usize) -> String {
        self.cells
            .get(&(row, col))
            .map(|cell| cell.value.to_string())
            .unwrap_or_default()
    }

    /// Get column width
    pub fn col_width(&self, col: usize) -> f32 {
        if col < self.cols {
            self.col_widths[col]
        } else {
            self.default_col_width
        }
    }

    /// Set column width
    pub fn set_col_width(&mut self, col: usize, width: f32) {
        if col < self.cols {
            self.col_widths[col] = width.max(20.0); // Minimum width
        }
    }

    /// Get row height
    pub fn row_height(&self, row: usize) -> f32 {
        if row < self.rows {
            self.row_heights[row]
        } else {
            self.default_row_height
        }
    }

    /// Set row height
    pub fn set_row_height(&mut self, row: usize, height: f32) {
        if row < self.rows {
            self.row_heights[row] = height.max(15.0); // Minimum height
        }
    }

    /// Calculate X position of column
    pub fn col_x_position(&self, col: usize) -> f32 {
        (0..col).map(|c| self.col_width(c)).sum()
    }

    /// Calculate Y position of row
    pub fn row_y_position(&self, row: usize) -> f32 {
        (0..row).map(|r| self.row_height(r)).sum()
    }

    /// Get total grid width
    pub fn total_width(&self) -> f32 {
        self.col_widths.iter().sum()
    }

    /// Get total grid height
    pub fn total_height(&self) -> f32 {
        self.row_heights.iter().sum()
    }

    /// Resize grid
    pub fn resize(&mut self, rows: usize, cols: usize) {
        // Remove cells outside new bounds
        self.cells.retain(|&(r, c), _| r < rows && c < cols);

        // Adjust column widths
        if cols > self.cols {
            self.col_widths.resize(cols, self.default_col_width);
        } else {
            self.col_widths.truncate(cols);
        }

        // Adjust row heights
        if rows > self.rows {
            self.row_heights.resize(rows, self.default_row_height);
        } else {
            self.row_heights.truncate(rows);
        }

        self.rows = rows;
        self.cols = cols;
    }

    /// Clear all cells
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /// Fill grid with sample data (for testing)
    pub fn fill_sample_data(&mut self) {
        // Header row
        for col in 0..self.cols {
            let header = format!("Column {}", col + 1);
            self.set_value(0, col, CellValue::Text(header));
        }

        // Data rows
        for row in 1..self.rows {
            for col in 0..self.cols {
                let value = if col == 0 {
                    CellValue::Text(format!("Row {}", row))
                } else {
                    CellValue::Number((row * self.cols + col) as f64)
                };
                self.set_value(row, col, value);
            }
        }
    }

    /// Get column name (A, B, C, ... Z, AA, AB, ...)
    pub fn get_col_name(col: usize) -> String {
        let mut result = String::new();
        let mut n = col;

        loop {
            let remainder = n % 26;
            result.insert(0, (b'A' + remainder as u8) as char);
            if n < 26 {
                break;
            }
            n = n / 26 - 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(100, 50);
        assert_eq!(grid.row_count(), 100);
        assert_eq!(grid.col_count(), 50);
    }

    #[test]
    fn test_cell_operations() {
        let mut grid = Grid::new(10, 10);

        grid.set_value(5, 5, CellValue::Text("test".to_string()));
        assert_eq!(grid.get_value_string(5, 5), "test");

        grid.set_value(3, 3, CellValue::Number(42.0));
        assert_eq!(grid.get_value_string(3, 3), "42");
    }
}

use super::cell::{Cell, CellValue, DataType};
use std::collections::{HashMap, HashSet};

/// Column configuration
#[derive(Clone, Debug)]
pub struct ColumnConfig {
    pub display_name: String,    // Display name shown in header
    pub internal_name: String,   // Internal unique identifier
    pub width: f32,              // Column width in pixels
    pub data_type: DataType,     // Data type (Text, Number, Date, Boolean)
    pub editable: bool,          // Can cells in this column be edited
    pub visible: bool,           // Is column visible
    pub sortable: bool,          // Can column be sorted
    pub filterable: bool,        // Can column be filtered
}

impl ColumnConfig {
    pub fn new(display_name: String, internal_name: String) -> Self {
        Self {
            display_name,
            internal_name,
            width: 100.0,
            data_type: DataType::Text,
            editable: true,
            visible: true,
            sortable: true,
            filterable: true,
        }
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn with_data_type(mut self, data_type: DataType) -> Self {
        self.data_type = data_type;
        self
    }

    pub fn with_editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }
}

/// Main grid data structure optimized for sparse data
pub struct Grid {
    rows: usize,
    cols: usize,
    // Sparse storage: only store non-empty cells
    // Key: (row, col), Value: Cell
    cells: HashMap<(usize, usize), Cell>,

    // Column configurations
    pub column_configs: Vec<ColumnConfig>,

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

    // Sort state
    pub sort_column: Option<usize>,
    pub sort_ascending: bool,
    pub sort_columns: Vec<(usize, bool)>, // Multi-column sort: (col, ascending)

    // Freeze state
    pub frozen_rows: usize,
    pub frozen_cols: usize,

    // Filter state
    filtered_rows: HashSet<usize>, // Rows that are hidden by filters

    // Grid-wide options
    pub readonly: bool,              // Read-only mode (no editing)
    pub enable_context_menu: bool,   // Enable right-click context menu
    pub enable_row_selection: bool,  // Allow row selection
    pub enable_col_selection: bool,  // Allow column selection
    pub show_grid_lines: bool,       // Show grid lines
    pub alternate_row_colors: bool,  // Alternate row background colors
}

impl Grid {
    /// Create a new grid with specified dimensions
    pub fn new(rows: usize, cols: usize) -> Self {
        let default_col_width = 100.0;
        let default_row_height = 25.0;

        // Create default column configs
        let column_configs = (0..cols)
            .map(|i| {
                let col_letter = Self::column_index_to_letter(i);
                ColumnConfig::new(col_letter.clone(), format!("col_{}", i))
            })
            .collect();

        Self {
            rows,
            cols,
            cells: HashMap::new(),
            column_configs,
            col_widths: vec![default_col_width; cols],
            row_heights: vec![default_row_height; rows],
            default_col_width,
            default_row_height,
            row_header_width: 60.0,
            col_header_height: 30.0,
            show_headers: true,
            sort_column: None,
            sort_ascending: true,
            sort_columns: Vec::new(),
            frozen_rows: 0,
            frozen_cols: 0,
            filtered_rows: HashSet::new(),
            readonly: false,
            enable_context_menu: true,
            enable_row_selection: true,
            enable_col_selection: true,
            show_grid_lines: true,
            alternate_row_colors: false,
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

    /// Insert a new row at the specified position
    pub fn insert_row(&mut self, at_index: usize) {
        if at_index > self.rows {
            return;
        }

        // Move all cells at or after the insertion point down by one row
        let mut new_cells = HashMap::new();
        for ((row, col), cell) in self.cells.drain() {
            if row >= at_index {
                new_cells.insert((row + 1, col), cell);
            } else {
                new_cells.insert((row, col), cell);
            }
        }
        self.cells = new_cells;

        // Insert new row height
        self.row_heights.insert(at_index, self.default_row_height);
        self.rows += 1;
    }

    /// Delete a row at the specified position
    pub fn delete_row(&mut self, index: usize) {
        if index >= self.rows || self.rows <= 1 {
            return;
        }

        // Remove cells in the deleted row and shift remaining cells up
        let mut new_cells = HashMap::new();
        for ((row, col), cell) in self.cells.drain() {
            if row == index {
                // Skip cells in deleted row
                continue;
            } else if row > index {
                // Shift rows down
                new_cells.insert((row - 1, col), cell);
            } else {
                new_cells.insert((row, col), cell);
            }
        }
        self.cells = new_cells;

        // Remove row height
        if index < self.row_heights.len() {
            self.row_heights.remove(index);
        }
        self.rows -= 1;
    }

    /// Insert a new column at the specified position
    pub fn insert_column(&mut self, at_index: usize) {
        if at_index > self.cols {
            return;
        }

        // Move all cells at or after the insertion point right by one column
        let mut new_cells = HashMap::new();
        for ((row, col), cell) in self.cells.drain() {
            if col >= at_index {
                new_cells.insert((row, col + 1), cell);
            } else {
                new_cells.insert((row, col), cell);
            }
        }
        self.cells = new_cells;

        // Insert new column width
        self.col_widths.insert(at_index, self.default_col_width);
        self.cols += 1;
    }

    /// Delete a column at the specified position
    pub fn delete_column(&mut self, index: usize) {
        if index >= self.cols || self.cols <= 1 {
            return;
        }

        // Remove cells in the deleted column and shift remaining cells left
        let mut new_cells = HashMap::new();
        for ((row, col), cell) in self.cells.drain() {
            if col == index {
                // Skip cells in deleted column
                continue;
            } else if col > index {
                // Shift columns left
                new_cells.insert((row, col - 1), cell);
            } else {
                new_cells.insert((row, col), cell);
            }
        }
        self.cells = new_cells;

        // Remove column width
        if index < self.col_widths.len() {
            self.col_widths.remove(index);
        }
        self.cols -= 1;
    }

    /// Sort grid by column
    pub fn sort_by_column(&mut self, col: usize, ascending: bool) {
        if col >= self.cols {
            return;
        }

        // Store sort state
        self.sort_column = Some(col);
        self.sort_ascending = ascending;

        // Collect all row indices and their values in the sort column
        let mut row_values: Vec<(usize, CellValue)> = Vec::new();

        for row in 0..self.rows {
            let value = self.get_value(row, col).clone();
            row_values.push((row, value));
        }

        // Sort rows based on column values
        row_values.sort_by(|(_, a), (_, b)| {
            let cmp = match (a, b) {
                (CellValue::Number(na), CellValue::Number(nb)) => {
                    na.partial_cmp(nb).unwrap_or(std::cmp::Ordering::Equal)
                }
                (CellValue::Boolean(ba), CellValue::Boolean(bb)) => ba.cmp(bb),
                (CellValue::Text(ta), CellValue::Text(tb)) => ta.cmp(tb),
                (CellValue::Empty, CellValue::Empty) => std::cmp::Ordering::Equal,
                (CellValue::Empty, _) => std::cmp::Ordering::Greater, // Empty values go to the end
                (_, CellValue::Empty) => std::cmp::Ordering::Less,
                // Mixed types: Number < Boolean < Text
                (CellValue::Number(_), _) => std::cmp::Ordering::Less,
                (_, CellValue::Number(_)) => std::cmp::Ordering::Greater,
                (CellValue::Boolean(_), CellValue::Text(_)) => std::cmp::Ordering::Less,
                (CellValue::Text(_), CellValue::Boolean(_)) => std::cmp::Ordering::Greater,
            };

            if ascending {
                cmp
            } else {
                cmp.reverse()
            }
        });

        // Create mapping from old row to new row
        let mut row_mapping: HashMap<usize, usize> = HashMap::new();
        for (new_row, (old_row, _)) in row_values.iter().enumerate() {
            row_mapping.insert(*old_row, new_row);
        }

        // Remap all cells to new row positions
        let mut new_cells = HashMap::new();
        for ((old_row, col_idx), cell) in self.cells.drain() {
            if let Some(&new_row) = row_mapping.get(&old_row) {
                new_cells.insert((new_row, col_idx), cell);
            }
        }
        self.cells = new_cells;

        // Remap row heights
        let mut new_row_heights = vec![self.default_row_height; self.rows];
        for (old_row, new_row) in row_mapping {
            if old_row < self.row_heights.len() && new_row < new_row_heights.len() {
                new_row_heights[new_row] = self.row_heights[old_row];
            }
        }
        self.row_heights = new_row_heights;

        // Clear multi-column sort when single column sort is used
        self.sort_columns.clear();
    }

    /// Add column to multi-column sort (for Shift+Click)
    pub fn add_sort_column(&mut self, col: usize, ascending: bool) {
        if col >= self.cols {
            return;
        }

        // Check if column already in sort list
        if let Some(pos) = self.sort_columns.iter().position(|(c, _)| *c == col) {
            // Update sort direction
            self.sort_columns[pos] = (col, ascending);
        } else {
            // Add new column to sort list
            self.sort_columns.push((col, ascending));
        }

        // Update primary sort column for compatibility
        if !self.sort_columns.is_empty() {
            self.sort_column = Some(self.sort_columns[0].0);
            self.sort_ascending = self.sort_columns[0].1;
        }

        // Perform multi-column sort
        self.sort_by_multiple_columns();
    }

    /// Sort by multiple columns
    pub fn sort_by_multiple_columns(&mut self) {
        if self.sort_columns.is_empty() {
            return;
        }

        // Collect all row indices and their values in all sort columns
        let mut row_values: Vec<(usize, Vec<CellValue>)> = Vec::new();

        for row in 0..self.rows {
            let mut values = Vec::new();
            for (col, _) in &self.sort_columns {
                values.push(self.get_value(row, *col).clone());
            }
            row_values.push((row, values));
        }

        // Sort rows based on multiple column values
        row_values.sort_by(|(_, values_a), (_, values_b)| {
            for (i, (col, ascending)) in self.sort_columns.iter().enumerate() {
                if i >= values_a.len() || i >= values_b.len() {
                    break;
                }

                let a = &values_a[i];
                let b = &values_b[i];

                let cmp = match (a, b) {
                    (CellValue::Number(na), CellValue::Number(nb)) => {
                        na.partial_cmp(nb).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    (CellValue::Boolean(ba), CellValue::Boolean(bb)) => ba.cmp(bb),
                    (CellValue::Text(ta), CellValue::Text(tb)) => ta.cmp(tb),
                    (CellValue::Date(da), CellValue::Date(db)) => da.cmp(db),
                    (CellValue::Empty, CellValue::Empty) => std::cmp::Ordering::Equal,
                    (CellValue::Empty, _) => std::cmp::Ordering::Greater,
                    (_, CellValue::Empty) => std::cmp::Ordering::Less,
                    (CellValue::Number(_), _) => std::cmp::Ordering::Less,
                    (_, CellValue::Number(_)) => std::cmp::Ordering::Greater,
                    (CellValue::Date(_), CellValue::Text(_)) => std::cmp::Ordering::Less,
                    (CellValue::Date(_), CellValue::Boolean(_)) => std::cmp::Ordering::Less,
                    (CellValue::Text(_), CellValue::Date(_)) => std::cmp::Ordering::Greater,
                    (CellValue::Boolean(_), CellValue::Date(_)) => std::cmp::Ordering::Greater,
                    (CellValue::Boolean(_), CellValue::Text(_)) => std::cmp::Ordering::Less,
                    (CellValue::Text(_), CellValue::Boolean(_)) => std::cmp::Ordering::Greater,
                };

                let final_cmp = if *ascending { cmp } else { cmp.reverse() };

                if final_cmp != std::cmp::Ordering::Equal {
                    return final_cmp;
                }
            }

            std::cmp::Ordering::Equal
        });

        // Create mapping from old row to new row
        let mut row_mapping: HashMap<usize, usize> = HashMap::new();
        for (new_row, (old_row, _)) in row_values.iter().enumerate() {
            row_mapping.insert(*old_row, new_row);
        }

        // Remap all cells to new row positions
        let mut new_cells = HashMap::new();
        for ((old_row, col_idx), cell) in self.cells.drain() {
            if let Some(&new_row) = row_mapping.get(&old_row) {
                new_cells.insert((new_row, col_idx), cell);
            }
        }
        self.cells = new_cells;

        // Remap row heights
        let mut new_row_heights = vec![self.default_row_height; self.rows];
        for (old_row, new_row) in row_mapping {
            if old_row < self.row_heights.len() && new_row < new_row_heights.len() {
                new_row_heights[new_row] = self.row_heights[old_row];
            }
        }
        self.row_heights = new_row_heights;
    }

    /// Clear multi-column sort
    pub fn clear_multi_column_sort(&mut self) {
        self.sort_columns.clear();
        self.sort_column = None;
    }

    /// Get multi-column sort state
    pub fn get_sort_columns(&self) -> &[(usize, bool)] {
        &self.sort_columns
    }

    /// Apply filter to a column
    pub fn apply_column_filter<F>(&mut self, col: usize, predicate: F)
    where
        F: Fn(&CellValue) -> bool,
    {
        self.filtered_rows.clear();

        for row in 0..self.rows {
            let value = self.get_value(row, col);
            if !predicate(value) {
                self.filtered_rows.insert(row);
            }
        }
    }

    /// Clear all filters
    pub fn clear_filters(&mut self) {
        self.filtered_rows.clear();
    }

    /// Check if a row is filtered (hidden)
    pub fn is_row_filtered(&self, row: usize) -> bool {
        self.filtered_rows.contains(&row)
    }

    /// Get count of visible (non-filtered) rows
    pub fn visible_row_count(&self) -> usize {
        self.rows - self.filtered_rows.len()
    }

    /// Get frozen row bounds (y positions)
    pub fn frozen_row_bounds(&self) -> (f32, f32) {
        if self.frozen_rows == 0 {
            return (0.0, 0.0);
        }

        let mut y = 0.0;
        for row in 0..self.frozen_rows.min(self.rows) {
            y += self.row_height(row);
        }
        (0.0, y)
    }

    /// Get frozen column bounds (x positions)
    pub fn frozen_col_bounds(&self) -> (f32, f32) {
        if self.frozen_cols == 0 {
            return (0.0, 0.0);
        }

        let mut x = 0.0;
        for col in 0..self.frozen_cols.min(self.cols) {
            x += self.col_width(col);
        }
        (0.0, x)
    }
    /// Get all cells in a specific row (for undo/redo)
    pub fn get_row_cells(&self, row: usize) -> Vec<(usize, Cell)> {
        self.cells
            .iter()
            .filter_map(|((r, c), cell)| {
                if *r == row {
                    Some((*c, cell.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get all cells in a specific column (for undo/redo)
    pub fn get_column_cells(&self, col: usize) -> Vec<(usize, Cell)> {
        self.cells
            .iter()
            .filter_map(|((r, c), cell)| {
                if *c == col {
                    Some((*r, cell.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Restore cells for a specific row (for undo)
    pub fn restore_row_cells(&mut self, row: usize, cells: &[(usize, Cell)]) {
        for (col, cell) in cells {
            self.cells.insert((row, *col), cell.clone());
        }
    }

    /// Restore cells for a specific column (for undo)
    pub fn restore_column_cells(&mut self, col: usize, cells: &[(usize, Cell)]) {
        for (row, cell) in cells {
            self.cells.insert((*row, col), cell.clone());
        }
    }

    /// Convert column index to letter (0 -> A, 1 -> B, ..., 25 -> Z, 26 -> AA, ...)
    pub fn column_index_to_letter(index: usize) -> String {
        let mut result = String::new();
        let mut n = index + 1;

        while n > 0 {
            let rem = (n - 1) % 26;
            result.insert(0, (b'A' + rem as u8) as char);
            n = (n - 1) / 26;
        }

        result
    }

    /// Set column configuration by index
    pub fn set_column_config(&mut self, col: usize, config: ColumnConfig) {
        if col < self.cols {
            if col < self.column_configs.len() {
                self.column_configs[col] = config;
            } else {
                // Extend configs if needed
                while self.column_configs.len() < col {
                    let default = ColumnConfig::new(
                        Self::column_index_to_letter(self.column_configs.len()),
                        format!("col_{}", self.column_configs.len()),
                    );
                    self.column_configs.push(default);
                }
                self.column_configs.push(config);
            }

            // Update column width if specified
            if col < self.col_widths.len() {
                self.col_widths[col] = self.column_configs[col].width;
            }
        }
    }

    /// Get column configuration by index
    pub fn get_column_config(&self, col: usize) -> Option<&ColumnConfig> {
        self.column_configs.get(col)
    }

    /// Get column index by internal name
    pub fn get_column_by_name(&self, name: &str) -> Option<usize> {
        self.column_configs
            .iter()
            .position(|c| c.internal_name == name)
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

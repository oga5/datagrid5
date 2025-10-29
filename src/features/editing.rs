use crate::core::{Grid, Viewport};
use web_sys::MouseEvent;

/// Cell editing functionality for DataGrid
pub struct EditingState {
    pub is_editing: bool,
    pub editing_cell: Option<(usize, usize)>,
}

impl Default for EditingState {
    fn default() -> Self {
        Self {
            is_editing: false,
            editing_cell: None,
        }
    }
}

impl EditingState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start editing a cell
    pub fn start_edit(&mut self, row: usize, col: usize, grid: &Grid) -> bool {
        // Check if cell is valid
        if row >= grid.row_count() || col >= grid.col_count() {
            return false;
        }

        // Check if column is editable
        if !grid.is_column_editable(col) {
            web_sys::console::log_1(&format!("Column {} is read-only", col).into());
            return false;
        }

        // Check if cell is editable
        if let Some(cell) = grid.get_cell(row, col) {
            if !cell.editable {
                return false;
            }
        }

        self.is_editing = true;
        self.editing_cell = Some((row, col));

        web_sys::console::log_1(&format!("Started editing cell: ({}, {})", row, col).into());
        true
    }

    /// End editing mode
    pub fn end_edit(&mut self) {
        self.is_editing = false;
        self.editing_cell = None;
        web_sys::console::log_1(&"Ended editing".into());
    }

    /// Check if currently editing
    pub fn is_editing(&self) -> bool {
        self.is_editing
    }

    /// Get the currently editing cell
    pub fn editing_cell(&self) -> Option<(usize, usize)> {
        self.editing_cell
    }

    /// Update cell value during editing
    pub fn update_cell_value(&mut self, row: usize, col: usize, value: String, grid: &mut Grid) {
        if self.is_editing && self.editing_cell == Some((row, col)) {
            use crate::core::cell::CellValue;
            grid.set_value(row, col, CellValue::Text(value.clone()));
            web_sys::console::log_1(&format!("Updated cell ({}, {}) to: {}", row, col, value).into());
        }
    }

    /// Get cell position for editing (returns canvas coordinates)
    pub fn get_cell_edit_rect(&self, row: usize, col: usize, grid: &Grid, viewport: &Viewport) -> Vec<f32> {
        // Account for header offsets
        let x = grid.row_header_width + grid.col_x_position(col) - viewport.scroll_x;
        let y = grid.col_header_height + grid.row_y_position(row) - viewport.scroll_y;
        let width = grid.col_width(col);
        let height = grid.row_height(row);

        vec![x, y, width, height]
    }

    /// Handle double-click for editing
    pub fn handle_double_click(&mut self, event: MouseEvent, grid: &Grid, viewport: &Viewport) -> Option<Vec<usize>> {
        let x = event.offset_x() as f32;
        let y = event.offset_y() as f32;

        // Get cell at click position
        if let Some((row, col)) = viewport.canvas_to_cell(x, y, grid) {
            if self.start_edit(row, col, grid) {
                return Some(vec![row, col]);
            }
        }

        None
    }
}

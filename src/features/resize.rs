use crate::core::{Grid, Viewport};

const RESIZE_HANDLE_WIDTH: f32 = 5.0;

/// Column/Row resizing functionality for DataGrid
pub struct ResizeState {
    pub is_resizing: bool,
    pub resizing_column: Option<usize>,
    pub resizing_row: Option<usize>,
    pub resize_start_pos: f32,
    pub resize_start_size: f32,
}

impl Default for ResizeState {
    fn default() -> Self {
        Self {
            is_resizing: false,
            resizing_column: None,
            resizing_row: None,
            resize_start_pos: 0.0,
            resize_start_size: 0.0,
        }
    }
}

impl ResizeState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if mouse is over a resize handle
    /// Returns: "col" for column resize, "row" for row resize, "none" otherwise
    pub fn check_resize_handle(&self, x: f32, y: f32, grid: &Grid, viewport: &Viewport) -> String {
        let _grid_x = x + viewport.scroll_x;
        let _grid_y = y + viewport.scroll_y;

        // Column resize: only detect in column header area
        if y < grid.col_header_height {
            let mut col_x = grid.row_header_width;
            for col in 0..grid.col_count() {
                let width = grid.col_width(col);
                col_x += width;

                // Check if near right edge of column
                if (x - col_x).abs() < RESIZE_HANDLE_WIDTH {
                    return "col".to_string();
                }
            }
        }

        // Row resize: only detect in row header area
        if x < grid.row_header_width {
            let mut row_y = grid.col_header_height;
            for row in 0..grid.row_count() {
                let height = grid.row_height(row);
                row_y += height;

                // Check if near bottom edge of row
                if (y - row_y).abs() < RESIZE_HANDLE_WIDTH {
                    return "row".to_string();
                }
            }
        }

        "none".to_string()
    }

    /// Start column or row resize
    pub fn start_resize(&mut self, x: f32, y: f32, resize_type: &str, grid: &Grid) -> bool {
        if resize_type == "col" {
            // Column resize: find which column in header area
            let mut col_x = grid.row_header_width;
            for col in 0..grid.col_count() {
                let width = grid.col_width(col);
                col_x += width;

                if (x - col_x).abs() < RESIZE_HANDLE_WIDTH {
                    self.is_resizing = true;
                    self.resizing_column = Some(col);
                    self.resize_start_pos = x;
                    self.resize_start_size = width;
                    web_sys::console::log_1(&format!("Started resizing column {}", col).into());
                    return true;
                }
            }
        } else if resize_type == "row" {
            // Row resize: find which row in header area
            let mut row_y = grid.col_header_height;
            for row in 0..grid.row_count() {
                let height = grid.row_height(row);
                row_y += height;

                if (y - row_y).abs() < RESIZE_HANDLE_WIDTH {
                    self.is_resizing = true;
                    self.resizing_row = Some(row);
                    self.resize_start_pos = y;
                    self.resize_start_size = height;
                    web_sys::console::log_1(&format!("Started resizing row {}", row).into());
                    return true;
                }
            }
        }

        false
    }

    /// Update resize during drag
    pub fn update_resize(&mut self, x: f32, y: f32, grid: &mut Grid) {
        if !self.is_resizing {
            return;
        }

        if let Some(col) = self.resizing_column {
            let delta = x - self.resize_start_pos;
            let new_width = (self.resize_start_size + delta).max(30.0); // Minimum 30px
            grid.set_col_width(col, new_width);
        } else if let Some(row) = self.resizing_row {
            let delta = y - self.resize_start_pos;
            let new_height = (self.resize_start_size + delta).max(20.0); // Minimum 20px
            grid.set_row_height(row, new_height);
        }
    }

    /// End resize
    pub fn end_resize(&mut self) {
        if self.is_resizing {
            web_sys::console::log_1(&"Ended resizing".into());
        }
        self.is_resizing = false;
        self.resizing_column = None;
        self.resizing_row = None;
        self.resize_start_pos = 0.0;
        self.resize_start_size = 0.0;
    }

    /// Check if currently resizing
    pub fn is_resizing(&self) -> bool {
        self.is_resizing
    }
}

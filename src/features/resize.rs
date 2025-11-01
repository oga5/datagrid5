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
        // Column resize: only detect in column header area
        if y < grid.col_header_height {
            // Convert canvas x to grid x, considering scroll offset
            let _grid_x = x - grid.row_header_width + viewport.scroll_x;

            // Check visible columns only
            let first_col = viewport.first_visible_col;
            let last_col = viewport.last_visible_col.min(grid.col_count().saturating_sub(1));

            for col in first_col..=last_col {
                let col_grid_x = grid.col_x_position(col);
                let col_width = grid.col_width(col);
                let col_right_edge = col_grid_x + col_width;

                // Calculate canvas position of the right edge
                let canvas_edge_x = col_right_edge - viewport.scroll_x + grid.row_header_width;

                // Check if mouse is near the right edge on canvas
                if (x - canvas_edge_x).abs() < RESIZE_HANDLE_WIDTH && canvas_edge_x > grid.row_header_width {
                    return "col".to_string();
                }
            }
        }

        // Row resize: only detect in row header area
        if x < grid.row_header_width {
            // Convert canvas y to grid y, considering scroll offset
            let _grid_y = y - grid.col_header_height + viewport.scroll_y;

            // Check visible rows only
            let first_row = viewport.first_visible_row;
            let last_row = viewport.last_visible_row.min(grid.row_count().saturating_sub(1));

            for row in first_row..=last_row {
                let row_grid_y = grid.row_y_position(row);
                let row_height = grid.row_height(row);
                let row_bottom_edge = row_grid_y + row_height;

                // Calculate canvas position of the bottom edge
                let canvas_edge_y = row_bottom_edge - viewport.scroll_y + grid.col_header_height;

                // Check if mouse is near the bottom edge on canvas
                if (y - canvas_edge_y).abs() < RESIZE_HANDLE_WIDTH && canvas_edge_y > grid.col_header_height {
                    return "row".to_string();
                }
            }
        }

        "none".to_string()
    }

    /// Start column or row resize
    pub fn start_resize(&mut self, x: f32, y: f32, resize_type: &str, grid: &Grid, viewport: &Viewport) -> bool {
        if resize_type == "col" {
            // Column resize: find which column in header area
            let first_col = viewport.first_visible_col;
            let last_col = viewport.last_visible_col.min(grid.col_count().saturating_sub(1));

            for col in first_col..=last_col {
                let col_grid_x = grid.col_x_position(col);
                let col_width = grid.col_width(col);
                let col_right_edge = col_grid_x + col_width;

                // Calculate canvas position of the right edge
                let canvas_edge_x = col_right_edge - viewport.scroll_x + grid.row_header_width;

                if (x - canvas_edge_x).abs() < RESIZE_HANDLE_WIDTH && canvas_edge_x > grid.row_header_width {
                    self.is_resizing = true;
                    self.resizing_column = Some(col);
                    self.resize_start_pos = x;
                    self.resize_start_size = col_width;
                    log::debug!("Started resizing column {}", col);
                    return true;
                }
            }
        } else if resize_type == "row" {
            // Row resize: find which row in header area
            let first_row = viewport.first_visible_row;
            let last_row = viewport.last_visible_row.min(grid.row_count().saturating_sub(1));

            for row in first_row..=last_row {
                let row_grid_y = grid.row_y_position(row);
                let row_height = grid.row_height(row);
                let row_bottom_edge = row_grid_y + row_height;

                // Calculate canvas position of the bottom edge
                let canvas_edge_y = row_bottom_edge - viewport.scroll_y + grid.col_header_height;

                if (y - canvas_edge_y).abs() < RESIZE_HANDLE_WIDTH && canvas_edge_y > grid.col_header_height {
                    self.is_resizing = true;
                    self.resizing_row = Some(row);
                    self.resize_start_pos = y;
                    self.resize_start_size = row_height;
                    log::debug!("Started resizing row {}", row);
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
            log::debug!("Ended resizing");
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

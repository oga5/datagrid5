use super::grid::Grid;

/// Viewport manages the visible area of the grid for virtual scrolling
#[derive(Debug, Clone)]
pub struct Viewport {
    /// Canvas width in pixels
    pub canvas_width: f32,

    /// Canvas height in pixels
    pub canvas_height: f32,

    /// Horizontal scroll offset in pixels
    pub scroll_x: f32,

    /// Vertical scroll offset in pixels
    pub scroll_y: f32,

    /// First visible row index
    pub first_visible_row: usize,

    /// Last visible row index
    pub last_visible_row: usize,

    /// First visible column index
    pub first_visible_col: usize,

    /// Last visible column index
    pub last_visible_col: usize,
}

impl Viewport {
    /// Create a new viewport
    pub fn new(canvas_width: f32, canvas_height: f32) -> Self {
        Self {
            canvas_width,
            canvas_height,
            scroll_x: 0.0,
            scroll_y: 0.0,
            first_visible_row: 0,
            last_visible_row: 0,
            first_visible_col: 0,
            last_visible_col: 0,
        }
    }

    /// Update viewport dimensions
    pub fn resize(&mut self, width: f32, height: f32) {
        self.canvas_width = width;
        self.canvas_height = height;
    }

    /// Set scroll position
    pub fn set_scroll(&mut self, x: f32, y: f32, grid: &Grid) {
        let max_scroll_x = (grid.total_width() - self.canvas_width).max(0.0);
        let max_scroll_y = (grid.total_height() - self.canvas_height).max(0.0);

        self.scroll_x = x.max(0.0).min(max_scroll_x);
        self.scroll_y = y.max(0.0).min(max_scroll_y);
    }

    /// Scroll by delta
    pub fn scroll_by(&mut self, dx: f32, dy: f32, grid: &Grid) {
        self.set_scroll(self.scroll_x + dx, self.scroll_y + dy, grid);
    }

    /// Update visible range based on current scroll position
    pub fn update_visible_range(&mut self, grid: &Grid) {
        // Calculate visible row range
        let mut y = 0.0;
        self.first_visible_row = 0;
        self.last_visible_row = grid.row_count().saturating_sub(1);

        for row in 0..grid.row_count() {
            let row_height = grid.row_height(row);

            if y + row_height > self.scroll_y && self.first_visible_row == 0 {
                self.first_visible_row = row;
            }

            if y > self.scroll_y + self.canvas_height {
                self.last_visible_row = row;
                break;
            }

            y += row_height;
        }

        // Calculate visible column range
        let mut x = 0.0;
        self.first_visible_col = 0;
        self.last_visible_col = grid.col_count().saturating_sub(1);

        for col in 0..grid.col_count() {
            let col_width = grid.col_width(col);

            if x + col_width > self.scroll_x && self.first_visible_col == 0 {
                self.first_visible_col = col;
            }

            if x > self.scroll_x + self.canvas_width {
                self.last_visible_col = col;
                break;
            }

            x += col_width;
        }
    }

    /// Get visible row count
    pub fn visible_row_count(&self) -> usize {
        if self.last_visible_row >= self.first_visible_row {
            self.last_visible_row - self.first_visible_row + 1
        } else {
            0
        }
    }

    /// Get visible column count
    pub fn visible_col_count(&self) -> usize {
        if self.last_visible_col >= self.first_visible_col {
            self.last_visible_col - self.first_visible_col + 1
        } else {
            0
        }
    }

    /// Convert canvas coordinates to grid cell position
    pub fn canvas_to_cell(&self, canvas_x: f32, canvas_y: f32, grid: &Grid) -> Option<(usize, usize)> {
        // Subtract header offset if headers are shown
        let header_offset_x = if grid.show_headers { grid.row_header_width } else { 0.0 };
        let header_offset_y = if grid.show_headers { grid.col_header_height } else { 0.0 };

        let grid_x = canvas_x - header_offset_x + self.scroll_x;
        let grid_y = canvas_y - header_offset_y + self.scroll_y;

        // Find row
        let mut y = 0.0;
        let mut row = None;
        for r in 0..grid.row_count() {
            let height = grid.row_height(r);
            if grid_y >= y && grid_y < y + height {
                row = Some(r);
                break;
            }
            y += height;
        }

        // Find column
        let mut x = 0.0;
        let mut col = None;
        for c in 0..grid.col_count() {
            let width = grid.col_width(c);
            if grid_x >= x && grid_x < x + width {
                col = Some(c);
                break;
            }
            x += width;
        }

        match (row, col) {
            (Some(r), Some(c)) => Some((r, c)),
            _ => None,
        }
    }

    /// Convert grid cell position to canvas coordinates
    pub fn cell_to_canvas(&self, row: usize, col: usize, grid: &Grid) -> (f32, f32) {
        let x = grid.col_x_position(col) - self.scroll_x;
        let y = grid.row_y_position(row) - self.scroll_y;
        (x, y)
    }

    /// Check if cell is visible
    pub fn is_cell_visible(&self, row: usize, col: usize) -> bool {
        row >= self.first_visible_row
            && row <= self.last_visible_row
            && col >= self.first_visible_col
            && col <= self.last_visible_col
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_creation() {
        let viewport = Viewport::new(800.0, 600.0);
        assert_eq!(viewport.canvas_width, 800.0);
        assert_eq!(viewport.canvas_height, 600.0);
    }

    #[test]
    fn test_scroll_bounds() {
        let mut viewport = Viewport::new(800.0, 600.0);
        let grid = Grid::new(100, 50);

        // Try to scroll beyond bounds
        viewport.set_scroll(-100.0, -100.0, &grid);
        assert_eq!(viewport.scroll_x, 0.0);
        assert_eq!(viewport.scroll_y, 0.0);
    }
}

use crate::core::{Grid, Viewport};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// Font configuration for text rendering
#[derive(Debug, Clone)]
pub struct FontConfig {
    pub family: String,
    pub size: f32,
    pub weight: String,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: "Arial, sans-serif".to_string(),
            size: 12.0,
            weight: "400".to_string(),
        }
    }
}

/// Text renderer using Canvas 2D API
pub struct TextRenderer {
    context: CanvasRenderingContext2d,
    font_config: FontConfig,

    // Colors
    text_color: String,
    header_text_color: String,
    selected_bg_color: String,
    selected_text_color: String,

    // Cached font string
    font_string: String,
}

impl TextRenderer {
    /// Create a new text renderer
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, String> {
        let context = canvas
            .get_context("2d")
            .map_err(|_| "Failed to get 2D context")?
            .ok_or("2D context is None")?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| "Failed to cast to CanvasRenderingContext2d")?;

        let font_config = FontConfig::default();
        let font_string = Self::build_font_string(&font_config);

        context.set_font(&font_string);
        context.set_text_baseline("middle");

        Ok(Self {
            context,
            font_config,
            text_color: "#333333".to_string(),
            header_text_color: "#000000".to_string(),
            selected_bg_color: "rgba(102, 126, 234, 0.2)".to_string(),
            selected_text_color: "#000000".to_string(),
            font_string,
        })
    }

    /// Build font string for Canvas API
    fn build_font_string(config: &FontConfig) -> String {
        format!("{} {}px {}", config.weight, config.size, config.family)
    }

    /// Update font configuration
    pub fn set_font(&mut self, config: FontConfig) {
        self.font_string = Self::build_font_string(&config);
        self.context.set_font(&self.font_string);
        self.font_config = config;
    }

    /// Clear the canvas
    pub fn clear(&self, width: f32, height: f32) {
        self.context.clear_rect(0.0, 0.0, width as f64, height as f64);
    }

    /// Render all visible text in the grid
    pub fn render(&self, grid: &Grid, viewport: &Viewport) {
        // Clear canvas first
        self.clear(viewport.canvas_width, viewport.canvas_height);

        // Render headers if enabled
        if grid.show_headers {
            self.render_headers(grid, viewport);
        }

        let first_row = viewport.first_visible_row;
        let last_row = viewport.last_visible_row.min(grid.row_count().saturating_sub(1));
        let first_col = viewport.first_visible_col;
        let last_col = viewport.last_visible_col.min(grid.col_count().saturating_sub(1));

        // Render visible cells
        for row in first_row..=last_row {
            for col in first_col..=last_col {
                self.render_cell(grid, viewport, row, col);
            }
        }
    }

    /// Render a single cell's text
    fn render_cell(&self, grid: &Grid, viewport: &Viewport, row: usize, col: usize) {
        let text = grid.get_value_string(row, col);
        if text.is_empty() {
            return;
        }

        // Calculate cell position on canvas
        let grid_x = grid.col_x_position(col);
        let grid_y = grid.row_y_position(row);

        // Apply header offset if headers are shown
        let header_offset_x = if grid.show_headers { grid.row_header_width } else { 0.0 };
        let header_offset_y = if grid.show_headers { grid.col_header_height } else { 0.0 };

        let canvas_x = grid_x - viewport.scroll_x + header_offset_x;
        let canvas_y = grid_y - viewport.scroll_y + header_offset_y;

        let width = grid.col_width(col);
        let height = grid.row_height(row);

        // Check if cell is visible
        if canvas_x + width < 0.0 || canvas_x > viewport.canvas_width
            || canvas_y + height < 0.0 || canvas_y > viewport.canvas_height
        {
            return;
        }

        // Check if cell is selected
        let is_selected = grid
            .get_cell(row, col)
            .map(|cell| cell.selected)
            .unwrap_or(false);

        // Draw selection background
        if is_selected {
            self.context.set_fill_style(&self.selected_bg_color.clone().into());
            self.context.fill_rect(
                canvas_x as f64,
                canvas_y as f64,
                width as f64,
                height as f64,
            );
        }

        // Set text color
        let text_color = if is_selected {
            &self.selected_text_color
        } else if row == 0 {
            &self.header_text_color
        } else {
            &self.text_color
        };
        self.context.set_fill_style(&text_color.clone().into());

        // Text padding
        let padding = 4.0;
        let text_x = canvas_x + padding;
        let text_y = canvas_y + height / 2.0;

        // Save context for clipping
        let _ = self.context.save();

        // Set clipping region to cell bounds
        self.context.begin_path();
        self.context.rect(
            canvas_x as f64,
            canvas_y as f64,
            width as f64,
            height as f64,
        );
        self.context.clip();

        // Draw text
        let _ = self.context.fill_text(&text, text_x as f64, text_y as f64);

        // Restore context
        let _ = self.context.restore();
    }

    /// Render cell with selection highlight
    pub fn render_cell_selection(&self, grid: &Grid, viewport: &Viewport, row: usize, col: usize) {
        let grid_x = grid.col_x_position(col);
        let grid_y = grid.row_y_position(row);

        // Apply header offset if headers are shown
        let header_offset_x = if grid.show_headers { grid.row_header_width } else { 0.0 };
        let header_offset_y = if grid.show_headers { grid.col_header_height } else { 0.0 };

        let canvas_x = grid_x - viewport.scroll_x + header_offset_x;
        let canvas_y = grid_y - viewport.scroll_y + header_offset_y;

        let width = grid.col_width(col);
        let height = grid.row_height(row);

        // Draw selection highlight border
        self.context.set_stroke_style(&"#667eea".into());
        self.context.set_line_width(2.0);
        self.context.stroke_rect(
            canvas_x as f64,
            canvas_y as f64,
            width as f64,
            height as f64,
        );
    }

    /// Measure text width
    pub fn measure_text(&self, text: &str) -> f32 {
        self.context
            .measure_text(text)
            .map(|metrics| metrics.width() as f32)
            .unwrap_or(0.0)
    }

    /// Get font height
    pub fn font_height(&self) -> f32 {
        self.font_config.size * 1.2 // Line height factor
    }

    /// Set text color
    pub fn set_text_color(&mut self, color: String) {
        self.text_color = color;
    }

    /// Set header text color
    pub fn set_header_text_color(&mut self, color: String) {
        self.header_text_color = color;
    }

    /// Set selection colors
    pub fn set_selection_colors(&mut self, bg_color: String, text_color: String) {
        self.selected_bg_color = bg_color;
        self.selected_text_color = text_color;
    }

    /// Render row and column headers
    fn render_headers(&self, grid: &Grid, viewport: &Viewport) {
        let row_header_width = grid.row_header_width;
        let col_header_height = grid.col_header_height;

        // Header background color
        let header_bg = "#f0f0f0";
        let header_border = "#cccccc";

        // Draw top-left corner cell (all-select button area)
        self.context.set_fill_style(&header_bg.into());
        self.context.fill_rect(0.0, 0.0, row_header_width as f64, col_header_height as f64);

        // Border for corner
        self.context.set_stroke_style(&header_border.into());
        self.context.set_line_width(1.0);
        self.context.stroke_rect(0.0, 0.0, row_header_width as f64, col_header_height as f64);

        // Render column headers
        self.render_column_headers(grid, viewport, row_header_width, col_header_height, header_bg, header_border);

        // Render row headers
        self.render_row_headers(grid, viewport, row_header_width, col_header_height, header_bg, header_border);
    }

    /// Render column headers (A, B, C, ...)
    fn render_column_headers(
        &self,
        grid: &Grid,
        viewport: &Viewport,
        row_header_width: f32,
        col_header_height: f32,
        header_bg: &str,
        header_border: &str,
    ) {
        let first_col = viewport.first_visible_col;
        let last_col = viewport.last_visible_col.min(grid.col_count().saturating_sub(1));

        for col in first_col..=last_col {
            let grid_x = grid.col_x_position(col);
            let canvas_x = grid_x - viewport.scroll_x + row_header_width;
            let width = grid.col_width(col);

            // Skip if not visible
            if canvas_x + width < row_header_width || canvas_x > viewport.canvas_width {
                continue;
            }

            // Draw header background
            self.context.set_fill_style(&header_bg.into());
            self.context.fill_rect(
                canvas_x as f64,
                0.0,
                width as f64,
                col_header_height as f64,
            );

            // Draw header border
            self.context.set_stroke_style(&header_border.into());
            self.context.set_line_width(1.0);
            self.context.stroke_rect(
                canvas_x as f64,
                0.0,
                width as f64,
                col_header_height as f64,
            );

            // Draw column name (A, B, C, ...)
            let col_name = Grid::get_col_name(col);
            self.context.set_fill_style(&self.header_text_color.clone().into());
            self.context.set_text_align("center");

            let text_x = canvas_x + width / 2.0;
            let text_y = col_header_height / 2.0;

            let _ = self.context.fill_text(&col_name, text_x as f64, text_y as f64);

            // Reset text align
            self.context.set_text_align("left");
        }
    }

    /// Render row headers (1, 2, 3, ...)
    fn render_row_headers(
        &self,
        grid: &Grid,
        viewport: &Viewport,
        row_header_width: f32,
        col_header_height: f32,
        header_bg: &str,
        header_border: &str,
    ) {
        let first_row = viewport.first_visible_row;
        let last_row = viewport.last_visible_row.min(grid.row_count().saturating_sub(1));

        for row in first_row..=last_row {
            let grid_y = grid.row_y_position(row);
            let canvas_y = grid_y - viewport.scroll_y + col_header_height;
            let height = grid.row_height(row);

            // Skip if not visible
            if canvas_y + height < col_header_height || canvas_y > viewport.canvas_height {
                continue;
            }

            // Draw header background
            self.context.set_fill_style(&header_bg.into());
            self.context.fill_rect(
                0.0,
                canvas_y as f64,
                row_header_width as f64,
                height as f64,
            );

            // Draw header border
            self.context.set_stroke_style(&header_border.into());
            self.context.set_line_width(1.0);
            self.context.stroke_rect(
                0.0,
                canvas_y as f64,
                row_header_width as f64,
                height as f64,
            );

            // Draw row number (1, 2, 3, ...)
            let row_number = format!("{}", row + 1);
            self.context.set_fill_style(&self.header_text_color.clone().into());
            self.context.set_text_align("center");

            let text_x = row_header_width / 2.0;
            let text_y = canvas_y + height / 2.0;

            let _ = self.context.fill_text(&row_number, text_x as f64, text_y as f64);

            // Reset text align
            self.context.set_text_align("left");
        }
    }
}

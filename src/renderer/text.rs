use crate::core::{Grid, Viewport};
use crate::GridError;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// Convert u32 RGBA color (0xRRGGBBAA) to CSS rgba() string
fn u32_to_rgba_string(color: u32) -> String {
    let r = ((color >> 24) & 0xFF) as u8;
    let g = ((color >> 16) & 0xFF) as u8;
    let b = ((color >> 8) & 0xFF) as u8;
    let a = (color & 0xFF) as f32 / 255.0;
    format!("rgba({}, {}, {}, {})", r, g, b, a)
}

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
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, GridError> {
        let context = canvas
            .get_context("2d")
            .map_err(|_| GridError::RenderInitFailed {
                error: "Failed to get 2D context".to_string(),
            })?
            .ok_or_else(|| GridError::RenderInitFailed {
                error: "2D context is None".to_string(),
            })?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| GridError::RenderInitFailed {
                error: "Failed to cast to CanvasRenderingContext2d".to_string(),
            })?;

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
        self.render_with_search(grid, viewport, &[], None);
    }

    pub fn render_with_search(
        &self,
        grid: &Grid,
        viewport: &Viewport,
        search_results: &[(usize, usize)],
        current_search_index: Option<usize>
    ) {
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

        // Get frozen bounds
        let frozen_rows = grid.frozen_rows;
        let frozen_cols = grid.frozen_cols;

        // Render in 4 regions to support frozen rows/columns:
        // 1. Top-left: frozen rows × frozen cols (always visible)
        // 2. Top-right: frozen rows × scrollable cols
        // 3. Bottom-left: scrollable rows × frozen cols
        // 4. Bottom-right: scrollable rows × scrollable cols

        // Region 1: Frozen rows × Frozen cols (top-left)
        if frozen_rows > 0 && frozen_cols > 0 {
            for row in 0..frozen_rows.min(grid.row_count()) {
                if grid.is_row_filtered(row) { continue; }
                for col in 0..frozen_cols.min(grid.col_count()) {
                    self.render_cell_with_search_frozen(grid, viewport, row, col, search_results, current_search_index, true, true);
                }
            }
        }

        // Region 2: Frozen rows × Scrollable cols (top-right)
        if frozen_rows > 0 {
            for row in 0..frozen_rows.min(grid.row_count()) {
                if grid.is_row_filtered(row) { continue; }
                for col in first_col.max(frozen_cols)..=last_col {
                    self.render_cell_with_search_frozen(grid, viewport, row, col, search_results, current_search_index, true, false);
                }
            }
        }

        // Region 3: Scrollable rows × Frozen cols (bottom-left)
        if frozen_cols > 0 {
            for row in first_row.max(frozen_rows)..=last_row {
                if grid.is_row_filtered(row) { continue; }
                for col in 0..frozen_cols.min(grid.col_count()) {
                    self.render_cell_with_search_frozen(grid, viewport, row, col, search_results, current_search_index, false, true);
                }
            }
        }

        // Region 4: Scrollable rows × Scrollable cols (bottom-right)
        for row in first_row.max(frozen_rows)..=last_row {
            if grid.is_row_filtered(row) { continue; }
            for col in first_col.max(frozen_cols)..=last_col {
                self.render_cell_with_search_frozen(grid, viewport, row, col, search_results, current_search_index, false, false);
            }
        }
    }

    /// Render a single cell's text
    fn render_cell(&self, grid: &Grid, viewport: &Viewport, row: usize, col: usize) {
        self.render_cell_with_search(grid, viewport, row, col, &[], None);
    }

    /// Render cell with frozen row/column support
    fn render_cell_with_search_frozen(
        &self,
        grid: &Grid,
        viewport: &Viewport,
        row: usize,
        col: usize,
        search_results: &[(usize, usize)],
        current_search_index: Option<usize>,
        is_frozen_row: bool,
        is_frozen_col: bool,
    ) {
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

        // For frozen rows/cols, don't apply scroll offset
        let scroll_x = if is_frozen_col { 0.0 } else { viewport.scroll_x };
        let scroll_y = if is_frozen_row { 0.0 } else { viewport.scroll_y };

        let canvas_x = grid_x - scroll_x + header_offset_x;
        let canvas_y = grid_y - scroll_y + header_offset_y;

        let width = grid.col_width(col);
        let height = grid.row_height(row);

        // Check if cell is visible
        if canvas_x + width < 0.0 || canvas_x > viewport.canvas_width
            || canvas_y + height < 0.0 || canvas_y > viewport.canvas_height
        {
            return;
        }

        // Get cell data
        let cell = grid.get_cell(row, col);
        let is_selected = cell.map(|c| c.selected).unwrap_or(false);

        // Check if this cell is a search result
        let is_search_match = search_results.contains(&(row, col));
        let is_current_match = if let Some(idx) = current_search_index {
            idx < search_results.len() && search_results[idx] == (row, col)
        } else {
            false
        };

        // Draw cell background
        // Clip background to avoid drawing over row headers (horizontal)
        let bg_x = if canvas_x < header_offset_x {
            header_offset_x
        } else {
            canvas_x
        };
        let bg_width = if canvas_x < header_offset_x {
            (width - (header_offset_x - canvas_x) - 1.0).max(0.0)
        } else {
            (width - 1.0).max(0.0)
        };

        // Clip background to avoid drawing over column headers (vertical)
        let bg_y = if canvas_y < header_offset_y {
            header_offset_y
        } else {
            canvas_y
        };
        let bg_height = if canvas_y < header_offset_y {
            (height - (header_offset_y - canvas_y) - 1.0).max(0.0)
        } else {
            (height - 1.0).max(0.0)
        };

        // Priority order: selected > current_match > search_match > cell.bg_color > default white
        if is_selected {
            self.context.set_fill_style(&self.selected_bg_color.clone().into());
            self.context.fill_rect(bg_x as f64, bg_y as f64, bg_width as f64, bg_height as f64);
        } else if is_current_match {
            self.context.set_fill_style(&"rgba(255, 165, 0, 0.6)".into());
            self.context.fill_rect(bg_x as f64, bg_y as f64, bg_width as f64, bg_height as f64);
        } else if is_search_match {
            self.context.set_fill_style(&"rgba(255, 255, 0, 0.3)".into());
            self.context.fill_rect(bg_x as f64, bg_y as f64, bg_width as f64, bg_height as f64);
        } else if let Some(cell) = cell {
            if let Some(bg_color) = cell.bg_color {
                let bg_str = u32_to_rgba_string(bg_color);
                self.context.set_fill_style(&bg_str.into());
                self.context.fill_rect(bg_x as f64, bg_y as f64, bg_width as f64, bg_height as f64);
            } else {
                // Draw default white background for consistent grid line appearance
                self.context.set_fill_style(&"#FFFFFF".into());
                self.context.fill_rect(bg_x as f64, bg_y as f64, bg_width as f64, bg_height as f64);
            }
        } else {
            // Draw default white background for empty cells
            self.context.set_fill_style(&"#FFFFFF".into());
            self.context.fill_rect(bg_x as f64, bg_y as f64, bg_width as f64, bg_height as f64);
        }

        // Set text color
        let text_color = if let Some(cell) = cell {
            if let Some(fg_color) = cell.fg_color {
                u32_to_rgba_string(fg_color)
            } else if is_selected {
                self.selected_text_color.clone()
            } else if row == 0 {
                self.header_text_color.clone()
            } else {
                self.text_color.clone()
            }
        } else {
            self.text_color.clone()
        };
        self.context.set_fill_style(&text_color.into());

        // Set font style
        if let Some(cell) = cell {
            let mut font_weight = "400".to_string();
            let mut font_style = "normal".to_string();

            if cell.font_bold {
                font_weight = "700".to_string();
            }
            if cell.font_italic {
                font_style = "italic".to_string();
            }

            let font_string = format!("{} {} {}px {}", font_style, font_weight, self.font_config.size, self.font_config.family);
            self.context.set_font(&font_string);
        } else {
            self.context.set_font(&self.font_string);
        }

        // Draw text with padding and clipping
        let padding = 5.0;
        let text_x = canvas_x + padding;
        let text_y = canvas_y + height / 2.0 + self.font_config.size / 3.0;

        // Save canvas state and set up clipping region
        self.context.save();
        self.context.begin_path();

        // Calculate clipping region, avoiding header areas
        // For non-frozen columns, ensure text doesn't render over the row header area
        let clip_x = if !is_frozen_col {
            canvas_x.max(header_offset_x)
        } else {
            canvas_x
        };

        // For non-frozen rows, ensure text doesn't render over the column header area
        let clip_y = if !is_frozen_row {
            canvas_y.max(header_offset_y)
        } else {
            canvas_y
        };

        let clip_right = (canvas_x + width).min(viewport.canvas_width);
        let clip_bottom = (canvas_y + height).min(viewport.canvas_height);
        let clip_width = clip_right - clip_x;
        let clip_height = clip_bottom - clip_y;

        // Only draw if there's visible area
        if clip_width > 0.0 && clip_height > 0.0 {
            self.context.rect(clip_x as f64, clip_y as f64, clip_width as f64, clip_height as f64);
            self.context.clip();

            // Draw text within clipping region
            let _ = self.context.fill_text(&text, text_x as f64, text_y as f64);
        }

        // Restore canvas state (removes clipping)
        self.context.restore();

        // Reset font
        self.context.set_font(&self.font_string);
    }

    /// Render a single cell's text with search highlighting
    fn render_cell_with_search(
        &self,
        grid: &Grid,
        viewport: &Viewport,
        row: usize,
        col: usize,
        search_results: &[(usize, usize)],
        current_search_index: Option<usize>
    ) {
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

        // Get cell data
        let cell = grid.get_cell(row, col);
        let is_selected = cell.map(|c| c.selected).unwrap_or(false);

        // Check if this cell is a search result
        let is_search_match = search_results.contains(&(row, col));
        let is_current_match = if let Some(idx) = current_search_index {
            idx < search_results.len() && search_results[idx] == (row, col)
        } else {
            false
        };

        // Draw cell background (priority: selected > current_match > search_match > cell.bg_color)
        // Reduce width/height by 1px to show grid lines
        let bg_width = (width - 1.0).max(0.0);
        let bg_height = (height - 1.0).max(0.0);

        if is_selected {
            // Use selection background (highest priority)
            self.context.set_fill_style(&self.selected_bg_color.clone().into());
            self.context.fill_rect(
                canvas_x as f64,
                canvas_y as f64,
                bg_width as f64,
                bg_height as f64,
            );
        } else if is_current_match {
            // Current (active) search result - bright orange
            self.context.set_fill_style(&"rgba(255, 165, 0, 0.6)".into());
            self.context.fill_rect(
                canvas_x as f64,
                canvas_y as f64,
                bg_width as f64,
                bg_height as f64,
            );
        } else if is_search_match {
            // Other search results - light yellow
            self.context.set_fill_style(&"rgba(255, 255, 0, 0.3)".into());
            self.context.fill_rect(
                canvas_x as f64,
                canvas_y as f64,
                bg_width as f64,
                bg_height as f64,
            );
        } else if let Some(cell) = cell {
            if let Some(bg_color) = cell.bg_color {
                // Use custom background color
                let bg_str = u32_to_rgba_string(bg_color);
                self.context.set_fill_style(&bg_str.into());
                self.context.fill_rect(
                    canvas_x as f64,
                    canvas_y as f64,
                    bg_width as f64,
                    bg_height as f64,
                );
            } else {
                // Draw default white background for consistent grid line appearance
                self.context.set_fill_style(&"#FFFFFF".into());
                self.context.fill_rect(
                    canvas_x as f64,
                    canvas_y as f64,
                    bg_width as f64,
                    bg_height as f64,
                );
            }
        } else {
            // Draw default white background for empty cells
            self.context.set_fill_style(&"#FFFFFF".into());
            self.context.fill_rect(
                canvas_x as f64,
                canvas_y as f64,
                bg_width as f64,
                bg_height as f64,
            );
        }

        // Set text color (custom or default)
        let text_color = if let Some(cell) = cell {
            if let Some(fg_color) = cell.fg_color {
                u32_to_rgba_string(fg_color)
            } else if is_selected {
                self.selected_text_color.clone()
            } else if row == 0 {
                self.header_text_color.clone()
            } else {
                self.text_color.clone()
            }
        } else {
            self.text_color.clone()
        };
        self.context.set_fill_style(&text_color.into());

        // Set font style (bold/italic)
        if let Some(cell) = cell {
            if cell.font_bold || cell.font_italic {
                let mut font_style = String::new();
                if cell.font_italic {
                    font_style.push_str("italic ");
                }
                if cell.font_bold {
                    font_style.push_str("bold ");
                }
                font_style.push_str(&format!("{}px {}", self.font_config.size, self.font_config.family));
                self.context.set_font(&font_style);
            } else {
                self.context.set_font(&self.font_string);
            }
        } else {
            self.context.set_font(&self.font_string);
        }

        // Text padding
        let padding = 4.0;
        let text_x = canvas_x + padding;
        let text_y = canvas_y + height / 2.0;

        // Save context for clipping
        let _ = self.context.save();

        // Calculate clipping region, avoiding header areas
        let clip_x = canvas_x.max(header_offset_x);
        let clip_y = canvas_y.max(header_offset_y);
        let clip_right = (canvas_x + width).min(viewport.canvas_width);
        let clip_bottom = (canvas_y + height).min(viewport.canvas_height);
        let clip_width = clip_right - clip_x;
        let clip_height = clip_bottom - clip_y;

        // Only draw if there's visible area
        if clip_width > 0.0 && clip_height > 0.0 {
            // Set clipping region to cell bounds (avoiding headers)
            self.context.begin_path();
            self.context.rect(
                clip_x as f64,
                clip_y as f64,
                clip_width as f64,
                clip_height as f64,
            );
            self.context.clip();

            // Draw text
            let _ = self.context.fill_text(&text, text_x as f64, text_y as f64);
        }

        // Restore context (also restores font)
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
        self.context.fill_rect(0.0, 0.0,
            row_header_width as f64,
            col_header_height as f64);

        // Border for corner
        self.context.set_stroke_style(&header_border.into());
        self.context.set_line_width(1.0);
        self.context.stroke_rect(0.0, 0.0,
            row_header_width as f64,
            col_header_height as f64);

        // Render column headers
        self.render_column_headers(grid, viewport, row_header_width, col_header_height, header_bg, header_border);

        // Render row headers
        self.render_row_headers(grid, viewport, row_header_width, col_header_height, header_bg, header_border);
    }

    /// Render column headers (A, B, C, ...) with optional multi-level grouping
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

        // If we have column groups, render multi-level headers
        if grid.header_levels > 1 && !grid.column_groups.is_empty() {
            self.render_grouped_column_headers(grid, viewport, row_header_width, header_bg, header_border, first_col, last_col);
        } else {
            // Render simple single-level headers
            self.render_simple_column_headers(grid, viewport, row_header_width, col_header_height, header_bg, header_border, first_col, last_col);
        }
    }

    /// Render simple single-level column headers
    fn render_simple_column_headers(
        &self,
        grid: &Grid,
        viewport: &Viewport,
        row_header_width: f32,
        col_header_height: f32,
        header_bg: &str,
        header_border: &str,
        first_col: usize,
        last_col: usize,
    ) {
        for col in first_col..=last_col {
            let grid_x = grid.col_x_position(col);
            let canvas_x = grid_x - viewport.scroll_x + row_header_width;
            let width = grid.col_width(col);

            // Skip if not visible
            if canvas_x + width < row_header_width || canvas_x > viewport.canvas_width {
                continue;
            }

            // Clip header if it overlaps with top-left corner
            let draw_x = canvas_x.max(row_header_width);
            let draw_width = if canvas_x < row_header_width {
                // Part of header is under top-left corner, clip it
                (canvas_x + width - row_header_width).max(0.0)
            } else {
                width
            };

            if draw_width <= 0.0 {
                continue;
            }

            // Draw header background (clipped to not overlap top-left corner)
            self.context.set_fill_style(&header_bg.into());
            self.context.fill_rect(
                draw_x as f64,
                0.0,
                draw_width as f64,
                col_header_height as f64,
            );

            // Draw header border (clipped to not overlap top-left corner)
            self.context.set_stroke_style(&header_border.into());
            self.context.set_line_width(1.0);
            self.context.stroke_rect(
                draw_x as f64,
                0.0,
                draw_width as f64,
                col_header_height as f64,
            );

            // Draw column name from column config (custom name or default A, B, C, ...)
            let col_name = if col < grid.column_configs.len() {
                grid.column_configs[col].display_name.clone()
            } else {
                Grid::get_col_name(col)
            };

            // Add sort indicator if this column is sorted
            let display_text = if grid.sort_column == Some(col) {
                if grid.sort_ascending {
                    format!("{} ▲", col_name)
                } else {
                    format!("{} ▼", col_name)
                }
            } else {
                col_name
            };

            // Draw text - always centered in the original cell position, not the clipped area
            self.context.set_fill_style(&self.header_text_color.clone().into());
            self.context.set_text_align("center");
            self.context.set_font(&self.font_string);

            // Text should be centered in the original cell, not the clipped visible area
            let text_x = canvas_x + width / 2.0;
            let text_y = col_header_height / 2.0;

            // Draw text as long as any part of the cell is visible
            if canvas_x + width > row_header_width {
                // Use canvas clipping to prevent text from drawing over row headers
                self.context.save();
                self.context.begin_path();
                self.context.rect(
                    row_header_width as f64,
                    0.0,
                    (viewport.canvas_width - row_header_width) as f64,
                    col_header_height as f64,
                );
                self.context.clip();

                let _ = self.context.fill_text(&display_text, text_x as f64, text_y as f64);

                self.context.restore();
            }

            // Reset text align
            self.context.set_text_align("left");
        }
    }

    /// Render multi-level grouped column headers
    fn render_grouped_column_headers(
        &self,
        grid: &Grid,
        viewport: &Viewport,
        row_header_width: f32,
        header_bg: &str,
        header_border: &str,
        first_col: usize,
        last_col: usize,
    ) {
        let header_row_height = grid.header_row_height;

        // Render group headers for each level (0 to header_levels - 2)
        for level in 0..(grid.header_levels - 1) {
            let groups_at_level = grid.get_column_groups_at_level(level);
            let y_pos = (level as f32) * header_row_height;

            for group in groups_at_level {
                // Calculate group span width
                let mut group_width = 0.0;
                let mut group_start_x = 0.0;
                let mut found_start = false;

                for col in group.start_col..=group.end_col.min(grid.col_count().saturating_sub(1)) {
                    let col_width = grid.col_width(col);
                    group_width += col_width;

                    if !found_start {
                        group_start_x = grid.col_x_position(col);
                        found_start = true;
                    }
                }

                let canvas_x = group_start_x - viewport.scroll_x + row_header_width;

                // Skip if not visible
                if canvas_x + group_width < row_header_width || canvas_x > viewport.canvas_width {
                    continue;
                }

                // Clip to visible area
                let visible_x = canvas_x.max(row_header_width);
                let visible_width = (canvas_x + group_width).min(viewport.canvas_width) - visible_x;

                if visible_width <= 0.0 {
                    continue;
                }

                // Draw group background
                self.context.set_fill_style(&"#e8e8e8".into());
                self.context.fill_rect(
                    visible_x as f64,
                    y_pos as f64,
                    visible_width as f64,
                    header_row_height as f64,
                );

                // Draw group border
                self.context.set_stroke_style(&header_border.into());
                self.context.set_line_width(1.0);
                self.context.stroke_rect(
                    visible_x as f64,
                    y_pos as f64,
                    visible_width as f64,
                    header_row_height as f64,
                );

                // Draw group label (centered, with clipping)
                self.context.set_fill_style(&"#333333".into());
                self.context.set_text_align("center");
                self.context.set_font("bold 13px Arial");

                let text_x = canvas_x + group_width / 2.0;
                let text_y = y_pos + header_row_height / 2.0 + 4.0;

                // Use clipping to prevent text from drawing over row headers
                self.context.save();
                self.context.begin_path();
                self.context.rect(
                    row_header_width as f64,
                    y_pos as f64,
                    (viewport.canvas_width - row_header_width) as f64,
                    header_row_height as f64,
                );
                self.context.clip();

                let _ = self.context.fill_text(&group.label, text_x as f64, text_y as f64);

                self.context.restore();

                // Reset font to normal (not bold)
                self.context.set_font(&self.font_string);
            }
        }

        // Render individual column headers at the last level
        let col_header_y = ((grid.header_levels - 1) as f32) * header_row_height;

        for col in first_col..=last_col {
            let grid_x = grid.col_x_position(col);
            let canvas_x = grid_x - viewport.scroll_x + row_header_width;
            let width = grid.col_width(col);

            // Skip if not visible
            if canvas_x + width < row_header_width || canvas_x > viewport.canvas_width {
                continue;
            }

            // Clip header if it overlaps with row headers
            let draw_x = canvas_x.max(row_header_width);
            let draw_width = if canvas_x < row_header_width {
                (canvas_x + width - row_header_width).max(0.0)
            } else {
                width
            };

            if draw_width <= 0.0 {
                continue;
            }

            // Draw header background (clipped)
            self.context.set_fill_style(&header_bg.into());
            self.context.fill_rect(
                draw_x as f64,
                col_header_y as f64,
                draw_width as f64,
                header_row_height as f64,
            );

            // Draw header border (clipped)
            self.context.set_stroke_style(&header_border.into());
            self.context.set_line_width(1.0);
            self.context.stroke_rect(
                draw_x as f64,
                col_header_y as f64,
                draw_width as f64,
                header_row_height as f64,
            );

            // Draw column name
            let col_name = Grid::get_col_name(col);

            // Add sort indicator if this column is sorted
            let display_text = if grid.sort_column == Some(col) {
                if grid.sort_ascending {
                    format!("{} ▲", col_name)
                } else {
                    format!("{} ▼", col_name)
                }
            } else {
                col_name
            };

            self.context.set_fill_style(&self.header_text_color.clone().into());
            self.context.set_text_align("center");
            self.context.set_font(&self.font_string);

            // Text should be centered in the original cell, not the clipped area
            let text_x = canvas_x + width / 2.0;
            let text_y = col_header_y + header_row_height / 2.0 + 4.0;

            // Draw text as long as any part of the cell is visible
            if canvas_x + width > row_header_width {
                // Use clipping to prevent text from drawing over row headers
                self.context.save();
                self.context.begin_path();
                self.context.rect(
                    row_header_width as f64,
                    col_header_y as f64,
                    (viewport.canvas_width - row_header_width) as f64,
                    header_row_height as f64,
                );
                self.context.clip();

                let _ = self.context.fill_text(&display_text, text_x as f64, text_y as f64);

                self.context.restore();
            }

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

            // Clip row header to avoid drawing over column headers (top corner)
            let draw_y = if canvas_y < col_header_height {
                col_header_height
            } else {
                canvas_y
            };
            let draw_height = if canvas_y < col_header_height {
                (height - (col_header_height - canvas_y)).max(0.0)
            } else {
                height
            };

            if draw_height <= 0.0 {
                continue;
            }

            // Draw header background (clipped)
            self.context.set_fill_style(&header_bg.into());
            self.context.fill_rect(
                0.0,
                draw_y as f64,
                row_header_width as f64,
                draw_height as f64,
            );

            // Draw header border (clipped)
            self.context.set_stroke_style(&header_border.into());
            self.context.set_line_width(1.0);
            self.context.stroke_rect(
                0.0,
                draw_y as f64,
                row_header_width as f64,
                draw_height as f64,
            );

            // Draw row number (1, 2, 3, ...) with clipping
            let row_number = format!("{}", row + 1);
            self.context.set_fill_style(&self.header_text_color.clone().into());
            self.context.set_text_align("center");
            self.context.set_font(&self.font_string);

            let text_x = row_header_width / 2.0;
            let text_y = canvas_y + height / 2.0;

            // Draw text as long as any part of the row header is visible
            if canvas_y + height > col_header_height {
                // Use clipping to prevent text from drawing over column headers
                self.context.save();
                self.context.begin_path();
                self.context.rect(
                    0.0,
                    col_header_height as f64,
                    row_header_width as f64,
                    (viewport.canvas_height - col_header_height) as f64,
                );
                self.context.clip();

                let _ = self.context.fill_text(&row_number, text_x as f64, text_y as f64);

                self.context.restore();
            }

            // Reset text align
            self.context.set_text_align("left");
        }
    }
}

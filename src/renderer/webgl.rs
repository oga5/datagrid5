use super::shader::ShaderProgram;
use crate::core::{Grid, Viewport};
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, WebGlBuffer, WebGlRenderingContext, WebGlUniformLocation,
};

/// WebGL-based grid renderer
pub struct WebGLRenderer {
    context: WebGlRenderingContext,
    shader_program: ShaderProgram,
    position_buffer: WebGlBuffer,
    color_buffer: WebGlBuffer,

    // Uniform locations
    u_resolution: WebGlUniformLocation,
    u_translation: WebGlUniformLocation,

    // Attribute locations
    a_position: u32,
    a_color: u32,

    // Canvas dimensions
    canvas_width: f32,
    canvas_height: f32,
}

impl WebGLRenderer {
    /// Create a new WebGL renderer
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, String> {
        let context = canvas
            .get_context("webgl")
            .map_err(|_| "Failed to get WebGL context")?
            .ok_or("WebGL context is None")?
            .dyn_into::<WebGlRenderingContext>()
            .map_err(|_| "Failed to cast to WebGlRenderingContext")?;

        let shader_program = ShaderProgram::new(&context)?;

        context.use_program(Some(&shader_program.program));

        // Get attribute locations
        let a_position = context.get_attrib_location(&shader_program.program, "a_position") as u32;
        let a_color = context.get_attrib_location(&shader_program.program, "a_color") as u32;

        // Get uniform locations
        let u_resolution = context
            .get_uniform_location(&shader_program.program, "u_resolution")
            .ok_or("Failed to get u_resolution uniform location")?;

        let u_translation = context
            .get_uniform_location(&shader_program.program, "u_translation")
            .ok_or("Failed to get u_translation uniform location")?;

        // Create buffers
        let position_buffer = context
            .create_buffer()
            .ok_or("Failed to create position buffer")?;

        let color_buffer = context
            .create_buffer()
            .ok_or("Failed to create color buffer")?;

        let canvas_width = canvas.width() as f32;
        let canvas_height = canvas.height() as f32;

        // Set viewport
        context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

        // Enable blending for transparency
        context.enable(WebGlRenderingContext::BLEND);
        context.blend_func(
            WebGlRenderingContext::SRC_ALPHA,
            WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        Ok(Self {
            context,
            shader_program,
            position_buffer,
            color_buffer,
            u_resolution,
            u_translation,
            a_position,
            a_color,
            canvas_width,
            canvas_height,
        })
    }

    /// Resize the renderer
    pub fn resize(&mut self, width: f32, height: f32) {
        self.canvas_width = width;
        self.canvas_height = height;
        self.context
            .viewport(0, 0, width as i32, height as i32);
    }

    /// Clear the canvas
    pub fn clear(&self) {
        self.context.clear_color(1.0, 1.0, 1.0, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }

    /// Render the grid with freeze support
    pub fn render(&self, grid: &Grid, viewport: &Viewport) {
        self.clear();

        self.context.use_program(Some(&self.shader_program.program));

        // Set resolution uniform (same for all regions)
        self.context
            .uniform2f(Some(&self.u_resolution), self.canvas_width, self.canvas_height);

        // Calculate header offset
        let header_offset_x = if grid.show_headers { grid.row_header_width } else { 0.0 };
        let header_offset_y = if grid.show_headers { grid.col_header_height } else { 0.0 };

        let frozen_rows = grid.frozen_rows;
        let frozen_cols = grid.frozen_cols;

        // Render in 4 regions to support frozen rows/columns:
        // 1. Top-left: frozen rows × frozen cols (no scroll)
        // 2. Top-right: frozen rows × scrollable cols (horizontal scroll only)
        // 3. Bottom-left: scrollable rows × frozen cols (vertical scroll only)
        // 4. Bottom-right: scrollable rows × scrollable cols (both scroll)

        // Region 1: Frozen rows × Frozen cols (top-left) - no scroll
        if frozen_rows > 0 && frozen_cols > 0 {
            self.context.uniform2f(
                Some(&self.u_translation),
                header_offset_x,
                header_offset_y,
            );
            self.render_region(grid, viewport, 0, frozen_rows, 0, frozen_cols);
        }

        // Region 2: Frozen rows × Scrollable cols (top-right) - horizontal scroll
        if frozen_rows > 0 {
            self.context.uniform2f(
                Some(&self.u_translation),
                -viewport.scroll_x + header_offset_x,
                header_offset_y,
            );
            self.render_region(
                grid,
                viewport,
                0,
                frozen_rows,
                frozen_cols.max(viewport.first_visible_col),
                viewport.last_visible_col.min(grid.col_count().saturating_sub(1)) + 1,
            );
        }

        // Region 3: Scrollable rows × Frozen cols (bottom-left) - vertical scroll
        if frozen_cols > 0 {
            self.context.uniform2f(
                Some(&self.u_translation),
                header_offset_x,
                -viewport.scroll_y + header_offset_y,
            );
            self.render_region(
                grid,
                viewport,
                frozen_rows.max(viewport.first_visible_row),
                viewport.last_visible_row.min(grid.row_count().saturating_sub(1)) + 1,
                0,
                frozen_cols,
            );
        }

        // Region 4: Scrollable rows × Scrollable cols (bottom-right) - both scroll
        self.context.uniform2f(
            Some(&self.u_translation),
            -viewport.scroll_x + header_offset_x,
            -viewport.scroll_y + header_offset_y,
        );
        self.render_region(
            grid,
            viewport,
            frozen_rows.max(viewport.first_visible_row),
            viewport.last_visible_row.min(grid.row_count().saturating_sub(1)) + 1,
            frozen_cols.max(viewport.first_visible_col),
            viewport.last_visible_col.min(grid.col_count().saturating_sub(1)) + 1,
        );

        // Note: Text rendering will be done via Canvas 2D API overlay
    }

    /// Render a specific region of the grid
    fn render_region(
        &self,
        grid: &Grid,
        _viewport: &Viewport,
        row_start: usize,
        row_end: usize,
        col_start: usize,
        col_end: usize,
    ) {
        if row_start >= row_end || col_start >= col_end {
            return;
        }

        // Render grid lines for this region
        self.render_grid_lines_region(grid, row_start, row_end, col_start, col_end);

        // Render cell backgrounds for this region
        self.render_cell_backgrounds_region(grid, row_start, row_end, col_start, col_end);

        // Render cell borders for this region
        self.render_cell_borders_region(grid, row_start, row_end, col_start, col_end);
    }

    /// Render grid lines for a specific region
    fn render_grid_lines_region(
        &self,
        grid: &Grid,
        row_start: usize,
        row_end: usize,
        col_start: usize,
        col_end: usize,
    ) {
        let mut positions: Vec<f32> = Vec::new();
        let mut colors: Vec<f32> = Vec::new();

        let line_color = [0.8, 0.8, 0.8, 1.0]; // Light gray

        // Calculate region bounds
        let x_start = grid.col_x_position(col_start);
        let x_end = if col_end < grid.col_count() {
            grid.col_x_position(col_end)
        } else {
            grid.total_width()
        };
        let y_start = grid.row_y_position(row_start);
        let y_end = if row_end < grid.row_count() {
            grid.row_y_position(row_end)
        } else {
            grid.total_height()
        };

        // Vertical lines (columns)
        for col in col_start..=col_end.min(grid.col_count()) {
            let x = grid.col_x_position(col);

            // Line from top to bottom of region
            positions.extend_from_slice(&[x, y_start, x, y_end]);
            colors.extend_from_slice(&line_color);
            colors.extend_from_slice(&line_color);
        }

        // Horizontal lines (rows)
        for row in row_start..=row_end.min(grid.row_count()) {
            let y = grid.row_y_position(row);

            // Line from left to right of region
            positions.extend_from_slice(&[x_start, y, x_end, y]);
            colors.extend_from_slice(&line_color);
            colors.extend_from_slice(&line_color);
        }

        self.draw_lines(&positions, &colors);
    }

    /// Render cell backgrounds for a specific region
    fn render_cell_backgrounds_region(
        &self,
        grid: &Grid,
        row_start: usize,
        row_end: usize,
        col_start: usize,
        col_end: usize,
    ) {
        let mut positions: Vec<f32> = Vec::new();
        let mut colors: Vec<f32> = Vec::new();

        for row in row_start..row_end.min(grid.row_count()) {
            if grid.is_row_filtered(row) {
                continue;
            }

            for col in col_start..col_end.min(grid.col_count()) {
                let x = grid.col_x_position(col);
                let y = grid.row_y_position(row);
                let width = grid.col_width(col);
                let height = grid.row_height(row);

                // Get cell and check for background color or selection
                let cell = grid.get_cell(row, col);

                let bg_color = if let Some(cell) = cell {
                    if let Some(cell_color) = cell.bg_color {
                        // Convert u32 RGBA to float array
                        let r = ((cell_color >> 24) & 0xFF) as f32 / 255.0;
                        let g = ((cell_color >> 16) & 0xFF) as f32 / 255.0;
                        let b = ((cell_color >> 8) & 0xFF) as f32 / 255.0;
                        let a = (cell_color & 0xFF) as f32 / 255.0;
                        [r, g, b, a]
                    } else if cell.selected {
                        [0.8, 0.9, 1.0, 1.0] // Light blue selection
                    } else {
                        continue; // Skip cells without background
                    }
                } else {
                    // No cell exists at this position, skip
                    continue;
                };

                // Two triangles to form a rectangle
                let x1 = x;
                let y1 = y;
                let x2 = x + width;
                let y2 = y + height;

                positions.extend_from_slice(&[
                    x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2,
                ]);

                for _ in 0..6 {
                    colors.extend_from_slice(&bg_color);
                }
            }
        }

        if !positions.is_empty() {
            self.draw_triangles(&positions, &colors);
        }
    }

    /// Render cell borders for a specific region
    fn render_cell_borders_region(
        &self,
        grid: &Grid,
        row_start: usize,
        row_end: usize,
        col_start: usize,
        col_end: usize,
    ) {
        let mut positions: Vec<f32> = Vec::new();
        let mut colors: Vec<f32> = Vec::new();

        let border_color = [0.6, 0.6, 0.6, 1.0]; // Dark gray

        for row in row_start..row_end.min(grid.row_count()) {
            if grid.is_row_filtered(row) {
                continue;
            }

            for col in col_start..col_end.min(grid.col_count()) {
                let x = grid.col_x_position(col);
                let y = grid.row_y_position(row);
                let width = grid.col_width(col);
                let height = grid.row_height(row);

                // Right border
                positions.extend_from_slice(&[x + width, y, x + width, y + height]);
                colors.extend_from_slice(&border_color);
                colors.extend_from_slice(&border_color);

                // Bottom border
                positions.extend_from_slice(&[x, y + height, x + width, y + height]);
                colors.extend_from_slice(&border_color);
                colors.extend_from_slice(&border_color);
            }
        }

        if !positions.is_empty() {
            self.draw_lines(&positions, &colors);
        }
    }

    /// Draw lines
    fn draw_lines(&self, positions: &[f32], colors: &[f32]) {
        if positions.is_empty() {
            return;
        }

        // Set position buffer
        self.context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.position_buffer),
        );

        unsafe {
            let positions_array = js_sys::Float32Array::view(positions);
            self.context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &positions_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        self.context.vertex_attrib_pointer_with_i32(
            self.a_position,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.context.enable_vertex_attrib_array(self.a_position);

        // Set color buffer
        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.color_buffer));

        unsafe {
            let colors_array = js_sys::Float32Array::view(colors);
            self.context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &colors_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        self.context.vertex_attrib_pointer_with_i32(
            self.a_color,
            4,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.context.enable_vertex_attrib_array(self.a_color);

        // Draw
        self.context
            .draw_arrays(WebGlRenderingContext::LINES, 0, (positions.len() / 2) as i32);
    }

    /// Draw triangles
    fn draw_triangles(&self, positions: &[f32], colors: &[f32]) {
        if positions.is_empty() {
            return;
        }

        // Set position buffer
        self.context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.position_buffer),
        );

        unsafe {
            let positions_array = js_sys::Float32Array::view(positions);
            self.context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &positions_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        self.context.vertex_attrib_pointer_with_i32(
            self.a_position,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.context.enable_vertex_attrib_array(self.a_position);

        // Set color buffer
        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.color_buffer));

        unsafe {
            let colors_array = js_sys::Float32Array::view(colors);
            self.context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &colors_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        self.context.vertex_attrib_pointer_with_i32(
            self.a_color,
            4,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.context.enable_vertex_attrib_array(self.a_color);

        // Draw
        self.context.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (positions.len() / 2) as i32,
        );
    }
}

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

    /// Render the grid
    pub fn render(&self, grid: &Grid, viewport: &Viewport) {
        self.clear();

        self.context.use_program(Some(&self.shader_program.program));

        // Set uniforms
        self.context
            .uniform2f(Some(&self.u_resolution), self.canvas_width, self.canvas_height);
        self.context
            .uniform2f(Some(&self.u_translation), -viewport.scroll_x, -viewport.scroll_y);

        // Render grid lines
        self.render_grid_lines(grid, viewport);

        // Render cell backgrounds
        self.render_cell_backgrounds(grid, viewport);

        // Render cell borders
        self.render_cell_borders(grid, viewport);

        // Note: Text rendering will be done via Canvas 2D API overlay
    }

    /// Render grid lines
    fn render_grid_lines(&self, grid: &Grid, viewport: &Viewport) {
        let mut positions: Vec<f32> = Vec::new();
        let mut colors: Vec<f32> = Vec::new();

        let line_color = [0.8, 0.8, 0.8, 1.0]; // Light gray

        // Vertical lines (columns)
        let mut x = 0.0;
        for col in 0..=grid.col_count() {
            if col > 0 {
                x += grid.col_width(col - 1);
            }

            // Line from top to bottom
            positions.extend_from_slice(&[x, 0.0, x, grid.total_height()]);
            colors.extend_from_slice(&line_color);
            colors.extend_from_slice(&line_color);
        }

        // Horizontal lines (rows)
        let mut y = 0.0;
        for row in 0..=grid.row_count() {
            if row > 0 {
                y += grid.row_height(row - 1);
            }

            // Line from left to right
            positions.extend_from_slice(&[0.0, y, grid.total_width(), y]);
            colors.extend_from_slice(&line_color);
            colors.extend_from_slice(&line_color);
        }

        self.draw_lines(&positions, &colors);
    }

    /// Render cell backgrounds
    fn render_cell_backgrounds(&self, grid: &Grid, viewport: &Viewport) {
        let mut positions: Vec<f32> = Vec::new();
        let mut colors: Vec<f32> = Vec::new();

        // Header background (row 0)
        let header_color = [0.95, 0.95, 0.95, 1.0]; // Light gray
        let mut x = 0.0;
        for col in 0..grid.col_count() {
            let width = grid.col_width(col);
            let height = grid.row_height(0);

            // Two triangles to form a rectangle
            let x1 = x;
            let y1 = 0.0;
            let x2 = x + width;
            let y2 = height;

            positions.extend_from_slice(&[
                x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2,
            ]);

            for _ in 0..6 {
                colors.extend_from_slice(&header_color);
            }

            x += width;
        }

        self.draw_triangles(&positions, &colors);
    }

    /// Render cell borders
    fn render_cell_borders(&self, grid: &Grid, viewport: &Viewport) {
        let mut positions: Vec<f32> = Vec::new();
        let mut colors: Vec<f32> = Vec::new();

        let border_color = [0.6, 0.6, 0.6, 1.0]; // Dark gray

        for row in viewport.first_visible_row..=viewport.last_visible_row.min(grid.row_count() - 1)
        {
            for col in
                viewport.first_visible_col..=viewport.last_visible_col.min(grid.col_count() - 1)
            {
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

        self.draw_lines(&positions, &colors);
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

mod core;
mod input;
mod renderer;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, MouseEvent, WheelEvent};

use core::{Cell, CellValue, Grid, Viewport};
use input::MouseHandler;
use renderer::WebGLRenderer;

// Use wee_alloc as the global allocator for smaller WASM size
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Main DataGrid control
#[wasm_bindgen]
pub struct DataGrid {
    grid: Grid,
    viewport: Viewport,
    renderer: WebGLRenderer,
    mouse_handler: MouseHandler,
    canvas: HtmlCanvasElement,
}

#[wasm_bindgen]
impl DataGrid {
    /// Create a new DataGrid instance
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, rows: usize, cols: usize) -> Result<DataGrid, JsValue> {
        // Set panic hook for better error messages
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        let document = web_sys::window()
            .ok_or("No window")?
            .document()
            .ok_or("No document")?;

        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or("Canvas not found")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Element is not a canvas")?;

        let canvas_width = canvas.width() as f32;
        let canvas_height = canvas.height() as f32;

        let mut grid = Grid::new(rows, cols);
        grid.fill_sample_data();

        let mut viewport = Viewport::new(canvas_width, canvas_height);
        viewport.update_visible_range(&grid);

        let renderer = WebGLRenderer::new(&canvas)
            .map_err(|e| JsValue::from_str(&e))?;

        let mouse_handler = MouseHandler::new();

        Ok(DataGrid {
            grid,
            viewport,
            renderer,
            mouse_handler,
            canvas,
        })
    }

    /// Render the grid
    pub fn render(&self) {
        self.renderer.render(&self.grid, &self.viewport);
    }

    /// Resize the grid
    pub fn resize(&mut self, width: f32, height: f32) {
        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);
        self.renderer.resize(width, height);
        self.viewport.resize(width, height);
        self.viewport.update_visible_range(&self.grid);
    }

    /// Handle mouse wheel event for scrolling
    pub fn handle_wheel(&mut self, event: WheelEvent) {
        let delta_x = event.delta_x() as f32;
        let delta_y = event.delta_y() as f32;

        self.viewport.scroll_by(delta_x, delta_y, &self.grid);
        self.viewport.update_visible_range(&self.grid);
    }

    /// Handle mouse down event
    pub fn handle_mouse_down(&mut self, event: MouseEvent) {
        let x = event.offset_x() as f32;
        let y = event.offset_y() as f32;

        self.mouse_handler.mouse_down(x, y);

        // Check if clicked on a cell
        if let Some((row, col)) = self.viewport.canvas_to_cell(x, y, &self.grid) {
            self.mouse_handler.select_cell(row, col);
            web_sys::console::log_1(&format!("Selected cell: ({}, {})", row, col).into());
        }
    }

    /// Handle mouse up event
    pub fn handle_mouse_up(&mut self, _event: MouseEvent) {
        self.mouse_handler.mouse_up();
    }

    /// Handle mouse move event
    pub fn handle_mouse_move(&mut self, event: MouseEvent) {
        let x = event.offset_x() as f32;
        let y = event.offset_y() as f32;

        if let Some((dx, dy)) = self.mouse_handler.mouse_move(x, y) {
            // Pan the viewport when dragging
            self.viewport.scroll_by(-dx, -dy, &self.grid);
            self.viewport.update_visible_range(&self.grid);
        }
    }

    /// Set cell value
    pub fn set_cell_value(&mut self, row: usize, col: usize, value: &str) {
        // Try to parse as number
        if let Ok(num) = value.parse::<f64>() {
            self.grid.set_value(row, col, CellValue::Number(num));
        } else {
            self.grid.set_value(row, col, CellValue::Text(value.to_string()));
        }
    }

    /// Get cell value
    pub fn get_cell_value(&self, row: usize, col: usize) -> String {
        self.grid.get_value_string(row, col)
    }

    /// Get grid dimensions
    pub fn get_dimensions(&self) -> Vec<usize> {
        vec![self.grid.row_count(), self.grid.col_count()]
    }

    /// Get viewport info
    pub fn get_viewport_info(&self) -> String {
        format!(
            "Visible: rows {}-{}, cols {}-{}",
            self.viewport.first_visible_row,
            self.viewport.last_visible_row,
            self.viewport.first_visible_col,
            self.viewport.last_visible_col
        )
    }
}

/// Initialize the library
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"DataGrid5 initialized".into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.row_count(), 10);
        assert_eq!(grid.col_count(), 10);
    }
}

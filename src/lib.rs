mod core;
mod features;
mod input;
mod renderer;

use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, KeyboardEvent, MouseEvent, WheelEvent};

use core::{cell::CellBorder, Cell, CellValue, ColumnConfig, DataType, Grid, Viewport};
use features::{
    editing::EditingState, resize::ResizeState, search::SearchState,
    selection::SelectionState, undo_redo::UndoRedoState, EditAction, CellStyle,
};
use input::{KeyboardHandler, MouseHandler, NavigationCommand};
use renderer::{TextRenderer, WebGLRenderer};

// Use wee_alloc as the global allocator for smaller WASM size
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Main DataGrid control
#[wasm_bindgen]
pub struct DataGrid {
    grid: Grid,
    viewport: Viewport,
    webgl_renderer: WebGLRenderer,
    text_renderer: TextRenderer,
    mouse_handler: MouseHandler,
    keyboard_handler: KeyboardHandler,
    webgl_canvas: HtmlCanvasElement,
    text_canvas: HtmlCanvasElement,
    // Feature modules
    editing: EditingState,
    selection: SelectionState,
    resize: ResizeState,
    search: SearchState,
    undo_redo: UndoRedoState,
    // Performance monitoring
    fps_samples: Vec<f64>,      // Store last N frame times
    last_frame_time: f64,       // Timestamp of last frame
    frame_count: u32,           // Total frame count
    render_time_ms: f64,        // Last render time in ms
    // Differential rendering
    dirty_cells: HashSet<(usize, usize)>, // Cells that need re-rendering
    needs_full_render: bool,    // Flag to force full re-render
}

#[wasm_bindgen]
impl DataGrid {
    /// Create a new DataGrid from a container div ID with JSON options
    /// Creates canvases automatically inside the div
    pub fn from_container(container_id: &str, options_json: &str) -> Result<DataGrid, JsValue> {
        // Set panic hook for better error messages
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        let document = web_sys::window()
            .ok_or("No window")?
            .document()
            .ok_or("No document")?;

        // Get container div
        let container = document
            .get_element_by_id(container_id)
            .ok_or_else(|| JsValue::from_str(&format!("Container '{}' not found", container_id)))?;

        // Parse options
        let options: serde_json::Value = serde_json::from_str(options_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid JSON options: {}", e)))?;

        let rows = options["rows"].as_u64().unwrap_or(100) as usize;
        let cols = options["cols"].as_u64().unwrap_or(26) as usize;
        let width = options["width"].as_u64().unwrap_or(800) as u32;
        let height = options["height"].as_u64().unwrap_or(600) as u32;

        // Clear container
        container.set_inner_html("");

        // Set container style - don't use height: 100% as it causes layout issues with absolute positioned canvases
        let container_style = format!("position: relative; width: 100%; height: {}px;", height);
        container.set_attribute("style", &container_style)?;

        // Create WebGL canvas
        let webgl_canvas = document
            .create_element("canvas")
            .map_err(|_| "Failed to create WebGL canvas")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Failed to cast to canvas")?;

        webgl_canvas.set_width(width);
        webgl_canvas.set_height(height);
        webgl_canvas.set_attribute("style", "position: absolute; top: 0; left: 0; z-index: 1;")?;

        // Create text overlay canvas
        let text_canvas = document
            .create_element("canvas")
            .map_err(|_| "Failed to create text canvas")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Failed to cast to canvas")?;

        text_canvas.set_width(width);
        text_canvas.set_height(height);
        text_canvas.set_attribute("style", "position: absolute; top: 0; left: 0; z-index: 2; pointer-events: none;")?;

        // Append canvases to container
        container.append_child(&webgl_canvas)?;
        container.append_child(&text_canvas)?;

        let canvas_width = width as f32;
        let canvas_height = height as f32;

        let mut grid = Grid::new(rows, cols);

        // Parse column configurations
        if let Some(columns) = options["columns"].as_array() {
            for (col_idx, col_config) in columns.iter().enumerate() {
                if col_idx >= cols {
                    break;
                }

                let display_name = col_config["display_name"]
                    .as_str()
                    .unwrap_or(&Grid::column_index_to_letter(col_idx))
                    .to_string();

                let internal_name = col_config["internal_name"]
                    .as_str()
                    .unwrap_or(&format!("col_{}", col_idx))
                    .to_string();

                let col_width = col_config["width"].as_f64().unwrap_or(100.0) as f32;

                let data_type = match col_config["data_type"].as_str() {
                    Some("number") => DataType::Number,
                    Some("date") => DataType::Date,
                    Some("boolean") => DataType::Boolean,
                    _ => DataType::Text,
                };

                let editable = col_config["editable"].as_bool().unwrap_or(true);
                let visible = col_config["visible"].as_bool().unwrap_or(true);
                let sortable = col_config["sortable"].as_bool().unwrap_or(true);
                let filterable = col_config["filterable"].as_bool().unwrap_or(true);

                let mut config = ColumnConfig::new(display_name, internal_name);
                config.width = col_width;
                config.data_type = data_type;
                config.editable = editable;
                config.visible = visible;
                config.sortable = sortable;
                config.filterable = filterable;

                grid.set_column_config(col_idx, config);
            }
        }

        // Apply grid-wide options
        grid.frozen_rows = options["frozen_rows"].as_u64().unwrap_or(0) as usize;
        grid.frozen_cols = options["frozen_cols"].as_u64().unwrap_or(0) as usize;
        grid.readonly = options["readonly"].as_bool().unwrap_or(false);
        grid.show_headers = options["show_headers"].as_bool().unwrap_or(true);
        grid.show_grid_lines = options["show_grid_lines"].as_bool().unwrap_or(true);
        grid.enable_context_menu = options["enable_context_menu"].as_bool().unwrap_or(true);
        grid.enable_row_selection = options["enable_row_selection"].as_bool().unwrap_or(true);
        grid.enable_col_selection = options["enable_col_selection"].as_bool().unwrap_or(true);
        grid.alternate_row_colors = options["alternate_row_colors"].as_bool().unwrap_or(false);

        if let Some(row_header_width) = options["row_header_width"].as_f64() {
            grid.row_header_width = row_header_width as f32;
        }
        if let Some(col_header_height) = options["col_header_height"].as_f64() {
            grid.col_header_height = col_header_height as f32;
        }

        let mut viewport = Viewport::new(canvas_width, canvas_height);
        viewport.update_visible_range(&grid);

        let webgl_renderer = WebGLRenderer::new(&webgl_canvas)
            .map_err(|e| JsValue::from_str(&e))?;

        let text_renderer = TextRenderer::new(&text_canvas)
            .map_err(|e| JsValue::from_str(&e))?;

        let mouse_handler = MouseHandler::new();
        let keyboard_handler = KeyboardHandler::new();

        Ok(DataGrid {
            grid,
            viewport,
            webgl_renderer,
            text_renderer,
            mouse_handler,
            keyboard_handler,
            webgl_canvas,
            text_canvas,
            editing: EditingState::new(),
            selection: SelectionState::new(),
            resize: ResizeState::new(),
            search: SearchState::new(),
            undo_redo: UndoRedoState::new(),
            fps_samples: Vec::new(),
            last_frame_time: 0.0,
            frame_count: 0,
            render_time_ms: 0.0,
            dirty_cells: HashSet::new(),
            needs_full_render: true,
        })
    }

    /// Create a new DataGrid instance with two canvas IDs (WebGL and text overlay)
    #[wasm_bindgen(constructor)]
    pub fn new(webgl_canvas_id: &str, text_canvas_id: &str, rows: usize, cols: usize) -> Result<DataGrid, JsValue> {
        // Set panic hook for better error messages
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        let document = web_sys::window()
            .ok_or("No window")?
            .document()
            .ok_or("No document")?;

        // Get WebGL canvas
        let webgl_canvas = document
            .get_element_by_id(webgl_canvas_id)
            .ok_or("WebGL canvas not found")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "WebGL element is not a canvas")?;

        // Get text overlay canvas
        let text_canvas = document
            .get_element_by_id(text_canvas_id)
            .ok_or("Text canvas not found")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Text element is not a canvas")?;

        let canvas_width = webgl_canvas.width() as f32;
        let canvas_height = webgl_canvas.height() as f32;

        let grid = Grid::new(rows, cols);

        let mut viewport = Viewport::new(canvas_width, canvas_height);
        viewport.update_visible_range(&grid);

        let webgl_renderer = WebGLRenderer::new(&webgl_canvas)
            .map_err(|e| JsValue::from_str(&e))?;

        let text_renderer = TextRenderer::new(&text_canvas)
            .map_err(|e| JsValue::from_str(&e))?;

        let mouse_handler = MouseHandler::new();
        let keyboard_handler = KeyboardHandler::new();

        Ok(DataGrid {
            grid,
            viewport,
            webgl_renderer,
            text_renderer,
            mouse_handler,
            keyboard_handler,
            webgl_canvas,
            text_canvas,
            editing: EditingState::new(),
            selection: SelectionState::new(),
            resize: ResizeState::new(),
            search: SearchState::new(),
            undo_redo: UndoRedoState::new(),
            fps_samples: Vec::new(),
            last_frame_time: 0.0,
            frame_count: 0,
            render_time_ms: 0.0,
            dirty_cells: HashSet::new(),
            needs_full_render: true, // Start with full render
        })
    }

    /// Render the grid
    pub fn render(&self) {
        // Render WebGL layer (grid lines and backgrounds)
        self.webgl_renderer.render(&self.grid, &self.viewport);

        // Render text layer on top with search highlight info
        self.text_renderer.render_with_search(
            &self.grid,
            &self.viewport,
            &self.search.search_results,
            self.search.current_search_index
        );
    }

    /// Resize the grid
    pub fn resize(&mut self, width: f32, height: f32) {
        self.webgl_canvas.set_width(width as u32);
        self.webgl_canvas.set_height(height as u32);
        self.text_canvas.set_width(width as u32);
        self.text_canvas.set_height(height as u32);

        self.webgl_renderer.resize(width, height);
        self.viewport.resize(width, height);
        self.viewport.update_visible_range(&self.grid);

        // Mark for full re-render after resize
        self.needs_full_render = true;
    }

    /// Handle mouse wheel event for scrolling
    pub fn handle_wheel(&mut self, event: WheelEvent) {
        let delta_x = event.delta_x() as f32;
        let delta_y = event.delta_y() as f32;

        self.viewport.scroll_by(delta_x, delta_y, &self.grid);
        self.viewport.update_visible_range(&self.grid);
    }

    /// Handle mouse down event with modifier keys
    pub fn handle_mouse_down_with_modifiers(&mut self, event: MouseEvent, shift: bool, ctrl: bool) {
        let x = event.offset_x() as f32;
        let y = event.offset_y() as f32;

        // If currently editing, commit the edit before processing the click
        if self.is_editing() {
            self.end_edit();
        }

        // Check if clicked on column header (for sorting)
        if let Some(col) = self.viewport.canvas_to_column_header(x, y, &self.grid) {
            web_sys::console::log_1(&format!("Clicked column header: {}", col).into());
            self.toggle_column_sort(col);
            return;
        }

        // Check if clicked on row header (already handled for row selection)
        if let Some(row) = self.viewport.canvas_to_row_header(x, y, &self.grid) {
            // Row header click - select entire row
            self.select_row(row);
            return;
        }

        // Check if clicked on a cell
        if let Some((row, col)) = self.viewport.canvas_to_cell(x, y, &self.grid) {
            if shift {
                // Shift+Click: Range selection
                self.select_range(row, col);
                self.mouse_handler.start_selection(x, y);
            } else if ctrl {
                // Ctrl+Click: Toggle selection
                self.toggle_cell_selection(row, col);
                self.mouse_handler.mouse_down(x, y);
            } else {
                // Normal click: Single selection and start drag selection
                self.select_single_cell(row, col);
                self.mouse_handler.start_selection(x, y);
            }

            self.mouse_handler.select_cell(row, col);
            web_sys::console::log_1(&format!("Selected {} cells", self.selection.selected_cells.len()).into());
        } else {
            // Clicked outside grid, clear selection
            if !ctrl {
                self.clear_selection();
            }
            self.mouse_handler.selected_cell = None;
            self.mouse_handler.mouse_down(x, y);
        }
    }

    /// Handle mouse down event (legacy, for backward compatibility)
    pub fn handle_mouse_down(&mut self, event: MouseEvent) {
        self.handle_mouse_down_with_modifiers(event, false, false);
    }

    /// Handle mouse down at coordinates with modifier keys (for JavaScript)
    pub fn handle_mouse_down_at_with_modifiers(&mut self, x: f32, y: f32, shift: bool, ctrl: bool) {
        web_sys::console::log_1(&format!("[DEBUG RS] handle_mouse_down_at_with_modifiers: x={}, y={}", x, y).into());

        // If currently editing, commit the edit before processing the click
        if self.is_editing() {
            self.end_edit();
        }

        // Check if clicked on column header (for sorting)
        if let Some(col) = self.viewport.canvas_to_column_header(x, y, &self.grid) {
            web_sys::console::log_1(&format!("[DEBUG RS] Clicked column header: {}", col).into());
            self.toggle_column_sort(col);
            return;
        }

        // Check if clicked on row header (for row selection)
        if let Some(row) = self.viewport.canvas_to_row_header(x, y, &self.grid) {
            web_sys::console::log_1(&format!("[DEBUG RS] Clicked row header: {}", row).into());
            self.select_row(row);
            return;
        }

        // Check if clicked on a cell
        if let Some((row, col)) = self.viewport.canvas_to_cell(x, y, &self.grid) {
            web_sys::console::log_1(&format!("[DEBUG RS] Clicked cell: ({}, {}), shift={}, ctrl={}", row, col, shift, ctrl).into());
            if shift {
                // Shift+Click: Range selection
                self.select_range(row, col);
                self.mouse_handler.start_selection(x, y);
            } else if ctrl {
                // Ctrl+Click: Toggle selection
                self.toggle_cell_selection(row, col);
                self.mouse_handler.mouse_down(x, y);
            } else {
                // Normal click: Single selection and start drag selection
                web_sys::console::log_1(&format!("[DEBUG RS] Calling select_single_cell({}, {})", row, col).into());
                self.select_single_cell(row, col);
                web_sys::console::log_1(&format!("[DEBUG RS] After select_single_cell, selected_cells.len={}", self.selection.selected_cells.len()).into());
                self.mouse_handler.start_selection(x, y);
            }

            self.mouse_handler.select_cell(row, col);
            web_sys::console::log_1(&format!("[DEBUG RS] Selected {} cells", self.selection.selected_cells.len()).into());
        } else {
            web_sys::console::log_1(&format!("[DEBUG RS] Clicked outside grid area").into());
            // Clicked outside grid, clear selection
            if !ctrl {
                self.clear_selection();
            }
            self.mouse_handler.selected_cell = None;
            self.mouse_handler.mouse_down(x, y);
        }
    }

    /// Handle mouse down at specific coordinates
    pub fn handle_mouse_down_at(&mut self, x: f32, y: f32) {
        web_sys::console::log_1(&format!("handle_mouse_down_at called: x={}, y={}", x, y).into());

        // If currently editing, commit the edit before processing the click
        if self.is_editing() {
            self.end_edit();
        }

        // Check if clicked on column header (for sorting)
        if let Some(col) = self.viewport.canvas_to_column_header(x, y, &self.grid) {
            web_sys::console::log_1(&format!("Clicked column header: {}", col).into());
            self.toggle_column_sort(col);
            return;
        }

        // Check if clicked on row header (already handled for row selection)
        if let Some(row) = self.viewport.canvas_to_row_header(x, y, &self.grid) {
            web_sys::console::log_1(&format!("Clicked row header: {}", row).into());
            // Row header click - select entire row
            self.select_row(row);
            return;
        }

        // Check if clicked on a cell
        if let Some((row, col)) = self.viewport.canvas_to_cell(x, y, &self.grid) {
            web_sys::console::log_1(&format!("Clicked cell: ({}, {})", row, col).into());
            // Normal click: Single selection and start drag selection
            self.select_single_cell(row, col);
            self.mouse_handler.start_selection(x, y);
            self.mouse_handler.select_cell(row, col);
            web_sys::console::log_1(&format!("Selected {} cells", self.selection.selected_cells.len()).into());
        } else {
            web_sys::console::log_1(&"No cell found at coordinates".into());
            // Clicked outside grid, clear selection
            self.clear_selection();
            self.mouse_handler.selected_cell = None;
        }
    }

    /// Handle mouse up event
    pub fn handle_mouse_up(&mut self, _event: MouseEvent) {
        self.mouse_handler.mouse_up();
    }

    /// Handle mouse up at specific coordinates
    pub fn handle_mouse_up_at(&mut self, _x: f32, _y: f32) {
        self.mouse_handler.mouse_up();
    }

    /// Handle mouse move event
    pub fn handle_mouse_move(&mut self, event: MouseEvent) {
        let x = event.offset_x() as f32;
        let y = event.offset_y() as f32;

        if self.mouse_handler.is_selecting {
            // Drag selection: extend range to current cell
            if let Some((row, col)) = self.viewport.canvas_to_cell(x, y, &self.grid) {
                self.select_range(row, col);
                self.mouse_handler.last_x = x;
                self.mouse_handler.last_y = y;
            }
        }
    }

    /// Handle context menu (right-click) event
    /// Returns JSON with context info: {"type": "row"|"column"|"cell", "row": N, "col": N}
    /// Returns empty string if not on grid
    pub fn handle_context_menu(&self, event: MouseEvent) -> String {
        let x = event.offset_x() as f32;
        let y = event.offset_y() as f32;

        // Check if on row header
        if let Some(row) = self.viewport.canvas_to_row_header(x, y, &self.grid) {
            return format!(
                r#"{{"type":"row","row":{},"col":null}}"#,
                row
            );
        }

        // Check if on column header
        if let Some(col) = self.viewport.canvas_to_column_header(x, y, &self.grid) {
            return format!(
                r#"{{"type":"column","row":null,"col":{}}}"#,
                col
            );
        }

        // Check if on cell
        if let Some((row, col)) = self.viewport.canvas_to_cell(x, y, &self.grid) {
            return format!(
                r#"{{"type":"cell","row":{},"col":{}}}"#,
                row, col
            );
        }

        // Not on grid
        String::new()
    }

    /// Get row operations for context menu
    /// Returns available operations for the given row
    pub fn get_row_context_operations(&self, row: usize) -> Vec<String> {
        let mut operations = vec![
            "insert_row_above".to_string(),
            "insert_row_below".to_string(),
        ];

        if self.grid.row_count() > 1 {
            operations.push("delete_row".to_string());
        }

        operations.push("copy_row".to_string());
        operations.push("cut_row".to_string());

        if row > 0 {
            operations.push("move_row_up".to_string());
        }

        if row < self.grid.row_count() - 1 {
            operations.push("move_row_down".to_string());
        }

        operations
    }

    /// Execute row context menu operation
    pub fn execute_row_operation(&mut self, operation: &str, row: usize) -> Result<String, JsValue> {
        match operation {
            "insert_row_above" => {
                self.insert_row(row);
                Ok(format!("Inserted row at {}", row))
            }
            "insert_row_below" => {
                self.insert_row(row + 1);
                Ok(format!("Inserted row at {}", row + 1))
            }
            "delete_row" => {
                if self.grid.row_count() <= 1 {
                    return Err(JsValue::from_str("Cannot delete the last row"));
                }
                self.delete_row(row);
                Ok(format!("Deleted row {}", row))
            }
            "copy_row" => {
                let mut cells = Vec::new();
                for col in 0..self.grid.col_count() {
                    cells.push(self.grid.get_value_string(row, col));
                }
                Ok(cells.join("\t"))
            }
            "cut_row" => {
                let mut cells = Vec::new();
                for col in 0..self.grid.col_count() {
                    cells.push(self.grid.get_value_string(row, col));
                    self.grid.set_value(row, col, CellValue::Empty);
                    self.dirty_cells.insert((row, col));
                }
                Ok(cells.join("\t"))
            }
            "move_row_up" => {
                if row == 0 {
                    return Err(JsValue::from_str("Cannot move first row up"));
                }
                self.swap_rows(row, row - 1);
                Ok(format!("Moved row {} up", row))
            }
            "move_row_down" => {
                if row >= self.grid.row_count() - 1 {
                    return Err(JsValue::from_str("Cannot move last row down"));
                }
                self.swap_rows(row, row + 1);
                Ok(format!("Moved row {} down", row))
            }
            _ => Err(JsValue::from_str(&format!("Unknown operation: {}", operation)))
        }
    }

    /// Swap two rows
    fn swap_rows(&mut self, row1: usize, row2: usize) {
        if row1 >= self.grid.row_count() || row2 >= self.grid.row_count() {
            return;
        }

        for col in 0..self.grid.col_count() {
            let val1 = self.grid.get_value(row1, col).clone();
            let val2 = self.grid.get_value(row2, col).clone();
            self.grid.set_value(row1, col, val2);
            self.grid.set_value(row2, col, val1);
            self.dirty_cells.insert((row1, col));
            self.dirty_cells.insert((row2, col));
        }
    }

    /// Set cell value
    pub fn set_cell_value(&mut self, row: usize, col: usize, value: &str) {
        // Record old value for undo
        let old_value = self.grid.get_value(row, col).clone();

        // Try to parse as number
        let new_value = if let Ok(num) = value.parse::<f64>() {
            CellValue::Number(num)
        } else {
            CellValue::Text(value.to_string())
        };

        // Only record if value actually changed
        if old_value != new_value {
            self.grid.set_value(row, col, new_value.clone());

            // Mark cell as modified
            if let Some(cell) = self.grid.get_cell_mut(row, col) {
                cell.modified = true;
            }

            // Mark cell as dirty for differential rendering
            self.dirty_cells.insert((row, col));

            // Record action for undo
            let action = EditAction::SetValue {
                row,
                col,
                old_value,
                new_value,
            };

            self.undo_redo.undo_stack.push(action);

            // Clear redo stack on new edit
            self.undo_redo.redo_stack.clear();
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

    /// Get row count
    pub fn row_count(&self) -> usize {
        self.grid.row_count()
    }

    /// Get column count
    pub fn col_count(&self) -> usize {
        self.grid.col_count()
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

    /// Get visible cell range for lazy loading (returns [first_row, last_row, first_col, last_col])
    pub fn get_visible_range(&self) -> Vec<usize> {
        vec![
            self.viewport.first_visible_row,
            self.viewport.last_visible_row,
            self.viewport.first_visible_col,
            self.viewport.last_visible_col,
        ]
    }

    /// Get viewport information as JSON string
    /// Returns: "[canvas_width, canvas_height, scroll_y, scroll_x]"
    pub fn get_viewport_info_array(&self) -> String {
        format!(
            "[{},{},{},{}]",
            self.viewport.canvas_width,
            self.viewport.canvas_height,
            self.viewport.scroll_y,
            self.viewport.scroll_x
        )
    }

    /// Get maximum scroll values as JSON string
    /// Returns: "[max_scroll_x, max_scroll_y]"
    pub fn get_max_scroll(&self) -> String {
        let header_offset_x = if self.grid.show_headers { self.grid.row_header_width } else { 0.0 };
        let header_offset_y = if self.grid.show_headers { self.grid.col_header_height } else { 0.0 };

        let viewport_width = self.viewport.canvas_width - header_offset_x;
        let viewport_height = self.viewport.canvas_height - header_offset_y;

        let max_scroll_x = (self.grid.total_width() - viewport_width).max(0.0);
        let max_scroll_y = (self.grid.total_height() - viewport_height).max(0.0);

        format!("[{},{}]", max_scroll_x, max_scroll_y)
    }

    /// Get total content size (including headers) as JSON string
    /// Returns: "[total_width, total_height]"
    pub fn get_total_size(&self) -> String {
        let header_offset_x = if self.grid.show_headers { self.grid.row_header_width } else { 0.0 };
        let header_offset_y = if self.grid.show_headers { self.grid.col_header_height } else { 0.0 };

        let total_width = self.grid.total_width() + header_offset_x;
        let total_height = self.grid.total_height() + header_offset_y;

        format!("[{},{}]", total_width, total_height)
    }

    /// Set scroll position
    pub fn set_scroll(&mut self, x: f32, y: f32) {
        self.viewport.set_scroll(x, y, &self.grid);
        self.viewport.update_visible_range(&self.grid);
    }

    /// Set multiple cell values at once (for lazy loading/batch updates)
    /// Takes JSON array of [row, col, value_type, value_data]
    /// value_type: 0=empty, 1=text, 2=number, 3=boolean
    /// Example: "[[0, 0, 1, \"text\"], [1, 1, 2, \"123\"]]"
    pub fn set_cells_batch(&mut self, cells_data_json: &str) -> Result<(), JsValue> {
        let cells_data: Vec<Vec<String>> = serde_json::from_str(cells_data_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid cells_data JSON: {}", e)))?;

        for cell_data in cells_data {
            if cell_data.len() < 4 {
                continue;
            }

            let row = cell_data[0].parse::<usize>().unwrap_or(0);
            let col = cell_data[1].parse::<usize>().unwrap_or(0);
            let value_type = cell_data[2].parse::<u8>().unwrap_or(0);
            let value_data = &cell_data[3];

            if row >= self.grid.row_count() || col >= self.grid.col_count() {
                continue;
            }

            let cell_value = match value_type {
                1 => CellValue::Text(value_data.clone()),
                2 => CellValue::Number(value_data.parse::<f64>().unwrap_or(0.0)),
                3 => CellValue::Boolean(value_data == "true" || value_data == "1"),
                _ => CellValue::Empty,
            };

            self.grid.set_value(row, col, cell_value);
            self.dirty_cells.insert((row, col));
        }

        Ok(())
    }

    /// Load grid data from JSON
    /// Accepts JSON array: [{"row": 0, "col": 0, "value": "text"}, ...]
    /// Value can be string, number, boolean, date, or null (for empty)
    /// If column has data_type configured, value will be converted accordingly
    pub fn load_data_json(&mut self, data_json: &str) -> Result<(), JsValue> {
        web_sys::console::log_1(&format!("load_data_json called with {} bytes", data_json.len()).into());

        let data: Vec<serde_json::Value> = serde_json::from_str(data_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid JSON data: {}", e)))?;

        web_sys::console::log_1(&format!("Parsed {} cell data entries", data.len()).into());

        let mut loaded_count = 0;
        for cell_data in data {
            let row = cell_data["row"].as_u64().unwrap_or(0) as usize;
            let col = cell_data["col"].as_u64().unwrap_or(0) as usize;

            if row >= self.grid.row_count() || col >= self.grid.col_count() {
                web_sys::console::log_1(&format!("Skipping cell ({}, {}) - out of bounds", row, col).into());
                continue;
            }

            // Get column data type if configured
            let expected_type = self.grid.get_column_config(col)
                .map(|c| c.data_type.clone());

            let cell_value = match &cell_data["value"] {
                serde_json::Value::Null => CellValue::Empty,
                serde_json::Value::String(s) => {
                    // Convert based on column data type
                    match expected_type {
                        Some(DataType::Number) => {
                            if let Ok(n) = s.parse::<f64>() {
                                CellValue::Number(n)
                            } else {
                                CellValue::Text(s.clone())
                            }
                        }
                        Some(DataType::Date) => CellValue::Date(s.clone()),
                        Some(DataType::Boolean) => {
                            CellValue::Boolean(s == "true" || s == "1")
                        }
                        _ => CellValue::Text(s.clone()),
                    }
                }
                serde_json::Value::Number(n) => {
                    if let Some(f) = n.as_f64() {
                        CellValue::Number(f)
                    } else {
                        CellValue::Empty
                    }
                }
                serde_json::Value::Bool(b) => CellValue::Boolean(*b),
                _ => CellValue::Empty,
            };

            self.grid.set_value(row, col, cell_value.clone());
            self.dirty_cells.insert((row, col));
            loaded_count += 1;

            if loaded_count <= 5 {
                web_sys::console::log_1(&format!("Loaded cell ({}, {}): {:?}", row, col, cell_value).into());
            }
        }

        web_sys::console::log_1(&format!("load_data_json completed. Loaded {} cells, {} dirty cells", loaded_count, self.dirty_cells.len()).into());

        Ok(())
    }

    /// Load data for a specific range (for lazy loading)
    /// Returns true if data is already loaded, false if needs loading
    pub fn is_range_loaded(&self, start_row: usize, end_row: usize, start_col: usize, end_col: usize) -> bool {
        // Check if any cells in the range have data
        let mut has_data = false;
        for row in start_row..=end_row.min(self.grid.row_count().saturating_sub(1)) {
            for col in start_col..=end_col.min(self.grid.col_count().saturating_sub(1)) {
                if self.grid.get_cell(row, col).is_some() {
                    has_data = true;
                    break;
                }
            }
            if has_data {
                break;
            }
        }
        has_data
    }

    /// Handle keyboard event
    pub fn handle_keyboard(&mut self, event: KeyboardEvent) -> bool {
        let key = event.key();

        // Get navigation command from key
        if let Some(command) = self.keyboard_handler.handle_key(&key) {
            // Get current selected cell
            let current = self.mouse_handler.selected_cell;

            let new_selection = match command {
                NavigationCommand::MoveUp => {
                    current.and_then(|(row, col)| {
                        if row > 0 {
                            Some((row - 1, col))
                        } else {
                            None
                        }
                    })
                }
                NavigationCommand::MoveDown => {
                    current.and_then(|(row, col)| {
                        if row < self.grid.row_count() - 1 {
                            Some((row + 1, col))
                        } else {
                            None
                        }
                    })
                }
                NavigationCommand::MoveLeft => {
                    current.and_then(|(row, col)| {
                        if col > 0 {
                            Some((row, col - 1))
                        } else {
                            None
                        }
                    })
                }
                NavigationCommand::MoveRight => {
                    current.and_then(|(row, col)| {
                        if col < self.grid.col_count() - 1 {
                            Some((row, col + 1))
                        } else {
                            None
                        }
                    })
                }
                NavigationCommand::PageUp => {
                    current.map(|(row, col)| {
                        let page_size = self.viewport.visible_row_count();
                        let new_row = row.saturating_sub(page_size);
                        (new_row, col)
                    })
                }
                NavigationCommand::PageDown => {
                    current.map(|(row, col)| {
                        let page_size = self.viewport.visible_row_count();
                        let new_row = (row + page_size).min(self.grid.row_count() - 1);
                        (new_row, col)
                    })
                }
                NavigationCommand::Home => {
                    current.map(|(row, _)| (row, 0))
                }
                NavigationCommand::End => {
                    current.map(|(row, _)| (row, self.grid.col_count() - 1))
                }
                NavigationCommand::DocumentStart => {
                    Some((0, 0))
                }
                NavigationCommand::DocumentEnd => {
                    Some((self.grid.row_count() - 1, self.grid.col_count() - 1))
                }
                NavigationCommand::Delete => {
                    // Clear cell content for all selected cells
                    if !self.selection.selected_cells.is_empty() {
                        // Collect old values for undo
                        let mut cells_to_clear = Vec::new();
                        for (row, col) in self.selection.selected_cells.iter() {
                            let old_value = self.grid.get_value(*row, *col);
                            cells_to_clear.push((*row, *col, old_value));
                        }

                        // Clear all cells
                        for (row, col, _) in &cells_to_clear {
                            self.grid.set_value(*row, *col, CellValue::Empty);
                        }

                        // Record undo action
                        let action = EditAction::ClearCells { cells: cells_to_clear };
                        self.undo_redo.undo_stack.push(action);
                        self.undo_redo.redo_stack.clear();

                        web_sys::console::log_1(&format!("Cleared {} cell(s)", self.selection.selected_cells.len()).into());
                        return true; // Force render
                    } else if let Some((row, col)) = current {
                        // Single cell clear
                        let old_value = self.grid.get_value(row, col);
                        self.grid.set_value(row, col, CellValue::Empty);

                        // Record undo action
                        let action = EditAction::ClearCells {
                            cells: vec![(row, col, old_value)]
                        };
                        self.undo_redo.undo_stack.push(action);
                        self.undo_redo.redo_stack.clear();

                        web_sys::console::log_1(&format!("Cleared cell: ({}, {})", row, col).into());
                        return true; // Force render
                    }
                    None
                }
                NavigationCommand::Undo => {
                    if self.undo() {
                        web_sys::console::log_1(&"Undo action".into());
                        return true; // Force render
                    }
                    None
                }
                NavigationCommand::Redo => {
                    if self.redo() {
                        web_sys::console::log_1(&"Redo action".into());
                        return true; // Force render
                    }
                    None
                }
                NavigationCommand::Enter => {
                    // If editing, end edit mode first
                    if self.is_editing() {
                        self.end_edit();
                    }
                    // Move down to next row
                    current.and_then(|(row, col)| {
                        if row < self.grid.row_count() - 1 {
                            Some((row + 1, col))
                        } else {
                            None
                        }
                    })
                }
                NavigationCommand::Escape => {
                    // If editing, cancel edit mode without moving
                    if self.is_editing() {
                        self.end_edit();
                    }
                    None
                }
                NavigationCommand::Tab => {
                    // If editing, end edit mode first
                    if self.is_editing() {
                        self.end_edit();
                    }
                    // Move right to next column
                    current.and_then(|(row, col)| {
                        if col < self.grid.col_count() - 1 {
                            Some((row, col + 1))
                        } else {
                            None
                        }
                    })
                }
            };

            // Update selection if we have a new one
            if let Some((new_row, new_col)) = new_selection {
                // Clear previous selection
                if let Some((prev_row, prev_col)) = current {
                    if let Some(cell) = self.grid.get_cell_mut(prev_row, prev_col) {
                        cell.selected = false;
                    }
                }

                // Set new selection
                self.mouse_handler.select_cell(new_row, new_col);
                if let Some(cell) = self.grid.get_cell_mut(new_row, new_col) {
                    cell.selected = true;
                } else {
                    let mut cell = Cell::default();
                    cell.selected = true;
                    self.grid.set_cell(new_row, new_col, cell);
                }

                // Ensure selected cell is visible
                self.ensure_cell_visible(new_row, new_col);

                web_sys::console::log_1(&format!("Navigated to cell: ({}, {})", new_row, new_col).into());

                return true; // Event handled
            }
        }

        false // Event not handled
    }

    /// Handle keyboard event with modifier keys
    pub fn handle_keyboard_with_modifiers(&mut self, event: KeyboardEvent, ctrl: bool) -> bool {
        let key = event.key();
        let shift = event.shift_key();

        // Get navigation command with modifiers
        if let Some(command) = self.keyboard_handler.handle_key_with_modifiers(&key, ctrl, shift) {
            let current = self.mouse_handler.selected_cell;

            let new_selection = match command {
                NavigationCommand::MoveUp => {
                    current.and_then(|(row, col)| {
                        if row > 0 {
                            Some((row - 1, col))
                        } else {
                            None
                        }
                    })
                }
                NavigationCommand::MoveDown => {
                    current.and_then(|(row, col)| {
                        if row < self.grid.row_count() - 1 {
                            Some((row + 1, col))
                        } else {
                            None
                        }
                    })
                }
                NavigationCommand::MoveLeft => {
                    current.and_then(|(row, col)| {
                        if col > 0 {
                            Some((row, col - 1))
                        } else {
                            None
                        }
                    })
                }
                NavigationCommand::MoveRight => {
                    current.and_then(|(row, col)| {
                        if col < self.grid.col_count() - 1 {
                            Some((row, col + 1))
                        } else {
                            None
                        }
                    })
                }
                NavigationCommand::PageUp => {
                    current.map(|(row, col)| {
                        let page_size = self.viewport.visible_row_count();
                        let new_row = row.saturating_sub(page_size);
                        (new_row, col)
                    })
                }
                NavigationCommand::PageDown => {
                    current.map(|(row, col)| {
                        let page_size = self.viewport.visible_row_count();
                        let new_row = (row + page_size).min(self.grid.row_count() - 1);
                        (new_row, col)
                    })
                }
                NavigationCommand::Home => {
                    current.map(|(row, _)| (row, 0))
                }
                NavigationCommand::End => {
                    current.map(|(row, _)| (row, self.grid.col_count() - 1))
                }
                NavigationCommand::DocumentStart => {
                    Some((0, 0))
                }
                NavigationCommand::DocumentEnd => {
                    Some((self.grid.row_count() - 1, self.grid.col_count() - 1))
                }
                NavigationCommand::Delete => {
                    if let Some((row, col)) = current {
                        self.grid.set_value(row, col, CellValue::Empty);
                        web_sys::console::log_1(&format!("Cleared cell: ({}, {})", row, col).into());
                    }
                    None
                }
                NavigationCommand::Undo => {
                    if self.undo() {
                        web_sys::console::log_1(&"Undo action".into());
                        return true; // Force render
                    }
                    None
                }
                NavigationCommand::Redo => {
                    if self.redo() {
                        web_sys::console::log_1(&"Redo action".into());
                        return true; // Force render
                    }
                    None
                }
                NavigationCommand::Enter | NavigationCommand::Escape | NavigationCommand::Tab => {
                    None
                }
            };

            if let Some((new_row, new_col)) = new_selection {
                if shift {
                    // Shift is pressed: extend selection (range selection)
                    if self.selection.selection_anchor.is_none() {
                        // No anchor yet, set current cell as anchor
                        if let Some((row, col)) = current {
                            self.selection.selection_anchor = Some((row, col));
                        }
                    }
                    // Extend selection to new cell
                    self.select_range(new_row, new_col);
                } else {
                    // No shift: move selection to new cell
                    if let Some((prev_row, prev_col)) = current {
                        if let Some(cell) = self.grid.get_cell_mut(prev_row, prev_col) {
                            cell.selected = false;
                        }
                    }

                    self.select_single_cell(new_row, new_col);
                }

                self.mouse_handler.select_cell(new_row, new_col);
                self.ensure_cell_visible(new_row, new_col);
                web_sys::console::log_1(&format!("Navigated to cell: ({}, {})", new_row, new_col).into());

                return true;
            }
        }

        false
    }

    /// Handle keyboard with key string and modifier flags (called from JavaScript)
    pub fn handle_keyboard_with_modifiers_key(&mut self, key: &str, ctrl: bool, shift: bool) -> bool {
        // Get navigation command with modifiers
        if let Some(command) = self.keyboard_handler.handle_key_with_modifiers(key, ctrl, shift) {
            let current = self.mouse_handler.selected_cell;

            let new_selection = match command {
                NavigationCommand::MoveUp => {
                    current.and_then(|(row, col)| {
                        if row > 0 {
                            Some((row - 1, col))
                        } else {
                            None
                        }
                    })
                }
                NavigationCommand::MoveDown => {
                    current.and_then(|(row, col)| {
                        if row < self.grid.row_count() - 1 {
                            Some((row + 1, col))
                        } else {
                            None
                        }
                    })
                }
                NavigationCommand::MoveLeft => {
                    current.and_then(|(row, col)| {
                        if col > 0 {
                            Some((row, col - 1))
                        } else {
                            None
                        }
                    })
                }
                NavigationCommand::MoveRight => {
                    current.and_then(|(row, col)| {
                        if col < self.grid.col_count() - 1 {
                            Some((row, col + 1))
                        } else {
                            None
                        }
                    })
                }
                NavigationCommand::PageUp => {
                    current.map(|(row, col)| {
                        let page_size = self.viewport.visible_row_count();
                        let new_row = row.saturating_sub(page_size);
                        (new_row, col)
                    })
                }
                NavigationCommand::PageDown => {
                    current.map(|(row, col)| {
                        let page_size = self.viewport.visible_row_count();
                        let new_row = (row + page_size).min(self.grid.row_count() - 1);
                        (new_row, col)
                    })
                }
                NavigationCommand::Home => {
                    current.map(|(row, _)| (row, 0))
                }
                NavigationCommand::End => {
                    current.map(|(row, _)| (row, self.grid.col_count() - 1))
                }
                NavigationCommand::DocumentStart => {
                    Some((0, 0))
                }
                NavigationCommand::DocumentEnd => {
                    Some((self.grid.row_count() - 1, self.grid.col_count() - 1))
                }
                NavigationCommand::Delete => {
                    if let Some((row, col)) = current {
                        self.grid.set_value(row, col, CellValue::Empty);
                    }
                    None
                }
                NavigationCommand::Undo => {
                    if self.undo() {
                        web_sys::console::log_1(&"Undo action".into());
                        return true; // Force render
                    }
                    None
                }
                NavigationCommand::Redo => {
                    if self.redo() {
                        web_sys::console::log_1(&"Redo action".into());
                        return true; // Force render
                    }
                    None
                }
                NavigationCommand::Enter | NavigationCommand::Escape | NavigationCommand::Tab => {
                    None
                }
            };

            if let Some((new_row, new_col)) = new_selection {
                if shift {
                    // Shift is pressed: extend selection (range selection)
                    if self.selection.selection_anchor.is_none() {
                        // No anchor yet, set current cell as anchor
                        if let Some((row, col)) = current {
                            self.selection.selection_anchor = Some((row, col));
                        }
                    }
                    // Extend selection to new cell
                    self.select_range(new_row, new_col);
                } else {
                    // No shift: move selection to new cell
                    if let Some((prev_row, prev_col)) = current {
                        if let Some(cell) = self.grid.get_cell_mut(prev_row, prev_col) {
                            cell.selected = false;
                        }
                    }

                    self.select_single_cell(new_row, new_col);
                }

                self.mouse_handler.select_cell(new_row, new_col);
                self.ensure_cell_visible(new_row, new_col);

                return true;
            }
        }

        false
    }

    /// Ensure a cell is visible in the viewport
    fn ensure_cell_visible(&mut self, row: usize, col: usize) {
        let cell_x = self.grid.col_x_position(col);
        let cell_y = self.grid.row_y_position(row);
        let cell_width = self.grid.col_width(col);
        let cell_height = self.grid.row_height(row);

        let mut scroll_x = self.viewport.scroll_x;
        let mut scroll_y = self.viewport.scroll_y;

        // Account for row and column headers
        let row_header_width = self.grid.row_header_width;
        let col_header_height = self.grid.col_header_height;

        // Calculate visible area (excluding headers)
        let visible_width = self.viewport.canvas_width - row_header_width;
        let visible_height = self.viewport.canvas_height - col_header_height;

        // Check horizontal visibility
        // Cell position relative to visible area (after row header)
        let cell_screen_x = cell_x - scroll_x;

        if cell_screen_x < 0.0 {
            // Cell is scrolled off to the left - align to left edge of visible area
            scroll_x = cell_x;
        } else if cell_screen_x + cell_width > visible_width {
            // Cell is scrolled off to the right - align to right edge of visible area
            scroll_x = cell_x + cell_width - visible_width;
        }

        // Check vertical visibility
        // Cell position relative to visible area (after column header)
        let cell_screen_y = cell_y - scroll_y;

        if cell_screen_y < 0.0 {
            // Cell is scrolled off to the top - align to top edge of visible area
            scroll_y = cell_y;
        } else if cell_screen_y + cell_height > visible_height {
            // Cell is scrolled off to the bottom - align to bottom edge of visible area
            scroll_y = cell_y + cell_height - visible_height;
        }

        // Update scroll if changed
        if scroll_x != self.viewport.scroll_x || scroll_y != self.viewport.scroll_y {
            self.viewport.set_scroll(scroll_x, scroll_y, &self.grid);
            self.viewport.update_visible_range(&self.grid);
        }
    }

    /// Start editing a cell (called from JavaScript)
    pub fn start_edit(&mut self, row: usize, col: usize) -> bool {
        // Use the EditingState's start_edit method
        self.editing.start_edit(row, col, &self.grid)
    }

    /// End editing mode
    pub fn end_edit(&mut self) {
        // Use the EditingState's end_edit method
        self.editing.end_edit();
    }

    /// Check if currently editing
    pub fn is_editing(&self) -> bool {
        // Use the EditingState's is_editing method
        self.editing.is_editing()
    }

    /// Update cell value during editing
    pub fn update_cell_value(&mut self, row: usize, col: usize, value: String) {
        // Record old value for undo
        let old_value = self.grid.get_value(row, col);
        let new_value = CellValue::Text(value.clone());

        // Update the cell
        self.editing.update_cell_value(row, col, value, &mut self.grid);

        // Record undo action
        let action = EditAction::SetValue {
            row,
            col,
            old_value,
            new_value,
        };
        self.undo_redo.undo_stack.push(action);
        self.undo_redo.redo_stack.clear();
    }

    /// Get cell position for editing (returns canvas coordinates)
    pub fn get_cell_edit_rect(&self, row: usize, col: usize) -> Vec<f32> {
        // Use the EditingState's get_cell_edit_rect method
        self.editing.get_cell_edit_rect(row, col, &self.grid, &self.viewport)
    }

    /// Handle double-click for editing
    pub fn handle_double_click(&mut self, event: MouseEvent) -> Option<Vec<usize>> {
        let x = event.offset_x() as f32;
        let y = event.offset_y() as f32;

        // Get cell at click position
        if let Some((row, col)) = self.viewport.canvas_to_cell(x, y, &self.grid) {
            if self.start_edit(row, col) {
                return Some(vec![row, col]);
            }
        }

        None
    }

    /// Handle double-click at specific canvas coordinates (for wrapper use)
    pub fn handle_double_click_at(&mut self, x: f32, y: f32) -> Option<String> {
        // Get cell at click position
        if let Some((row, col)) = self.viewport.canvas_to_cell(x, y, &self.grid) {
            if self.start_edit(row, col) {
                return Some(format!("[{},{}]", row, col));
            }
        }

        None
    }

    /// Check if mouse is over a resize handle
    /// Returns: "col" for column resize, "row" for row resize, "none" otherwise
    pub fn check_resize_handle(&self, x: f32, y: f32) -> String {
        // Use ResizeState's check_resize_handle method
        self.resize.check_resize_handle(x, y, &self.grid, &self.viewport)
    }

    /// Start column or row resize
    pub fn start_resize(&mut self, x: f32, y: f32, resize_type: &str) -> bool {
        // Use ResizeState's start_resize method with viewport information
        self.resize.start_resize(x, y, resize_type, &self.grid, &self.viewport)
    }

    /// Update resize during drag
    pub fn update_resize(&mut self, x: f32, y: f32) {
        // Use ResizeState's update_resize method
        self.resize.update_resize(x, y, &mut self.grid);
    }

    /// End resize
    pub fn end_resize(&mut self) {
        // Use ResizeState's end_resize method
        self.resize.end_resize();
    }

    /// Check if currently resizing
    pub fn is_resizing(&self) -> bool {
        self.resize.is_resizing
    }

    /// Select a single cell (clears previous selection)
    fn select_single_cell(&mut self, row: usize, col: usize) {
        // Clear all previous selections
        self.clear_selection();

        // Add new selection
        self.selection.selected_cells.insert((row, col));
        self.selection.selection_anchor = Some((row, col));

        // Update cell state
        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            cell.selected = true;
        } else {
            let mut cell = Cell::default();
            cell.selected = true;
            self.grid.set_cell(row, col, cell);
        }
    }

    /// Toggle cell selection (add/remove from selection)
    fn toggle_cell_selection(&mut self, row: usize, col: usize) {
        if self.selection.selected_cells.contains(&(row, col)) {
            // Remove from selection
            self.selection.selected_cells.remove(&(row, col));
            if let Some(cell) = self.grid.get_cell_mut(row, col) {
                cell.selected = false;
            }
        } else {
            // Add to selection
            self.selection.selected_cells.insert((row, col));
            if let Some(cell) = self.grid.get_cell_mut(row, col) {
                cell.selected = true;
            } else {
                let mut cell = Cell::default();
                cell.selected = true;
                self.grid.set_cell(row, col, cell);
            }
        }

        // Update anchor
        if !self.selection.selected_cells.is_empty() {
            self.selection.selection_anchor = Some((row, col));
        }
    }

    /// Select range from anchor to target cell
    fn select_range(&mut self, target_row: usize, target_col: usize) {
        if let Some((anchor_row, anchor_col)) = self.selection.selection_anchor {
            // Clear previous selection
            self.clear_selection();

            // Calculate range
            let min_row = anchor_row.min(target_row);
            let max_row = anchor_row.max(target_row);
            let min_col = anchor_col.min(target_col);
            let max_col = anchor_col.max(target_col);

            // Select all cells in range
            for r in min_row..=max_row {
                for c in min_col..=max_col {
                    if r < self.grid.row_count() && c < self.grid.col_count() {
                        self.selection.selected_cells.insert((r, c));

                        if let Some(cell) = self.grid.get_cell_mut(r, c) {
                            cell.selected = true;
                        } else {
                            let mut cell = Cell::default();
                            cell.selected = true;
                            self.grid.set_cell(r, c, cell);
                        }
                    }
                }
            }
        } else {
            // No anchor, just select single cell
            self.select_single_cell(target_row, target_col);
        }
    }

    /// Clear all selections
    fn clear_selection(&mut self) {
        for (row, col) in &self.selection.selected_cells {
            if let Some(cell) = self.grid.get_cell_mut(*row, *col) {
                cell.selected = false;
            }
        }
        self.selection.selected_cells.clear();
    }

    /// Get selected cells as a JSON array of [row, col] pairs
    /// Returns: "[[row1, col1], [row2, col2], ...]"
    pub fn get_selected_cells(&self) -> String {
        let cells: Vec<Vec<usize>> = self.selection.selected_cells
            .iter()
            .map(|(row, col)| vec![*row, *col])
            .collect();
        serde_json::to_string(&cells).unwrap_or_else(|_| "[]".to_string())
    }

    /// Get selection count
    pub fn get_selection_count(&self) -> usize {
        self.selection.selected_cells.len()
    }

    /// Select a single cell and make it the active cell
    pub fn select_cell(&mut self, row: usize, col: usize) {
        // Clear previous selection
        self.clear_selection();

        // Select the cell
        self.selection.selected_cells.insert((row, col));
        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            cell.selected = true;
        }

        // Set as anchor
        self.selection.selection_anchor = Some((row, col));

        // Update mouse handler
        self.mouse_handler.select_cell(row, col);
    }

    /// Select all cells (Ctrl+A)
    pub fn select_all(&mut self) {
        self.clear_selection();

        for row in 0..self.grid.row_count() {
            for col in 0..self.grid.col_count() {
                self.selection.selected_cells.insert((row, col));
                if let Some(cell) = self.grid.get_cell_mut(row, col) {
                    cell.selected = true;
                }
            }
        }

        // Set anchor to first cell
        self.selection.selection_anchor = Some((0, 0));
    }

    /// Select entire row
    pub fn select_row(&mut self, row: usize) {
        if row >= self.grid.row_count() {
            return;
        }

        self.clear_selection();

        for col in 0..self.grid.col_count() {
            self.selection.selected_cells.insert((row, col));
            if let Some(cell) = self.grid.get_cell_mut(row, col) {
                cell.selected = true;
            }
        }

        self.selection.selection_anchor = Some((row, 0));
    }

    /// Select entire column
    pub fn select_col(&mut self, col: usize) {
        if col >= self.grid.col_count() {
            return;
        }

        self.clear_selection();

        for row in 0..self.grid.row_count() {
            self.selection.selected_cells.insert((row, col));
            if let Some(cell) = self.grid.get_cell_mut(row, col) {
                cell.selected = true;
            }
        }

        self.selection.selection_anchor = Some((0, col));
    }

    /// Copy selected cells to TSV (Tab-Separated Values) format
    /// Returns a string with cells separated by tabs and rows separated by newlines
    pub fn copy_selected_cells(&self) -> String {
        if self.selection.selected_cells.is_empty() {
            return String::new();
        }

        // Sort selected cells by row, then by column
        let mut cells: Vec<(usize, usize)> = self.selection.selected_cells.iter().copied().collect();
        cells.sort_by(|a, b| {
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });

        // Find the bounding box
        let min_row = cells.iter().map(|(r, _)| r).min().unwrap();
        let max_row = cells.iter().map(|(r, _)| r).max().unwrap();
        let min_col = cells.iter().map(|(_, c)| c).min().unwrap();
        let max_col = cells.iter().map(|(_, c)| c).max().unwrap();

        // Build TSV string
        let mut result = String::new();
        for row in *min_row..=*max_row {
            for col in *min_col..=*max_col {
                if self.selection.selected_cells.contains(&(row, col)) {
                    let value = self.grid.get_value_string(row, col);
                    result.push_str(&value);
                } else {
                    // Empty cell in the rectangular selection
                    result.push_str("");
                }

                if col < *max_col {
                    result.push('\t');
                }
            }
            if row < *max_row {
                result.push('\n');
            }
        }

        result
    }

    /// Cut selected cells (copy and then clear)
    pub fn cut_selected_cells(&mut self) -> String {
        // First copy the cells
        let clipboard_text = self.copy_selected_cells();

        // Then clear all selected cells
        let cells_to_clear: Vec<(usize, usize)> = self.selection.selected_cells.iter().copied().collect();
        for (row, col) in cells_to_clear {
            self.grid.set_value(row, col, CellValue::Empty);
        }

        clipboard_text
    }

    /// Paste cells from TSV (Tab-Separated Values) format
    /// Pastes starting from the current focus cell
    pub fn paste_cells(&mut self, tsv_text: String) -> Result<(), String> {
        if tsv_text.is_empty() {
            return Ok(());
        }

        // Determine starting position (focus cell or first selected cell)
        let (start_row, start_col) = if let Some(anchor) = self.selection.selection_anchor {
            anchor
        } else if !self.selection.selected_cells.is_empty() {
            let mut cells: Vec<(usize, usize)> = self.selection.selected_cells.iter().copied().collect();
            cells.sort_by(|a, b| {
                if a.0 == b.0 {
                    a.1.cmp(&b.1)
                } else {
                    a.0.cmp(&b.0)
                }
            });
            cells[0]
        } else {
            return Err("No cell selected for paste".to_string());
        };

        // Parse TSV and paste, recording old and new values for undo/redo
        let mut changed_cells = Vec::new();
        let lines: Vec<&str> = tsv_text.lines().collect();

        for (row_offset, line) in lines.iter().enumerate() {
            let target_row = start_row + row_offset;
            if target_row >= self.grid.row_count() {
                break; // Don't paste beyond grid bounds
            }

            let values: Vec<&str> = line.split('\t').collect();
            for (col_offset, value) in values.iter().enumerate() {
                let target_col = start_col + col_offset;
                if target_col >= self.grid.col_count() {
                    break; // Don't paste beyond grid bounds
                }

                // Record old value for undo
                let old_value = self.grid.get_value(target_row, target_col);

                // Create cell value
                let new_value = if value.is_empty() {
                    CellValue::Empty
                } else if let Ok(num) = value.parse::<f64>() {
                    CellValue::Number(num)
                } else if *value == "true" || *value == "false" {
                    CellValue::Boolean(*value == "true")
                } else {
                    CellValue::Text(value.to_string())
                };

                self.grid.set_value(target_row, target_col, new_value.clone());
                changed_cells.push((target_row, target_col, old_value, new_value));
            }
        }

        // Record undo action for all pasted cells
        if !changed_cells.is_empty() {
            let action = EditAction::SetMultipleCells { cells: changed_cells };
            self.undo_redo.undo_stack.push(action);
            self.undo_redo.redo_stack.clear();
        }

        Ok(())
    }

    /// Helper: Get current cell style (for undo tracking)
    fn get_cell_style(&self, row: usize, col: usize) -> CellStyle {
        if let Some(cell) = self.grid.get_cell(row, col) {
            CellStyle {
                bg_color: cell.bg_color,
                fg_color: cell.fg_color,
                font_bold: cell.font_bold,
                font_italic: cell.font_italic,
            }
        } else {
            CellStyle {
                bg_color: None,
                fg_color: None,
                font_bold: false,
                font_italic: false,
            }
        }
    }

    /// Set background color for a cell (RGBA as u32: 0xRRGGBBAA)
    pub fn set_cell_bg_color(&mut self, row: usize, col: usize, color: u32) {
        let old_style = self.get_cell_style(row, col);

        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            cell.bg_color = Some(color);
        } else {
            // Create cell if it doesn't exist
            let mut cell = Cell::empty();
            cell.bg_color = Some(color);
            self.grid.set_cell(row, col, cell);
        }

        let new_style = self.get_cell_style(row, col);
        let action = EditAction::SetStyle { row, col, old_style, new_style };
        self.undo_redo.undo_stack.push(action);
        self.undo_redo.redo_stack.clear();
    }

    /// Set foreground (text) color for a cell (RGBA as u32: 0xRRGGBBAA)
    pub fn set_cell_fg_color(&mut self, row: usize, col: usize, color: u32) {
        let old_style = self.get_cell_style(row, col);

        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            cell.fg_color = Some(color);
        } else {
            let mut cell = Cell::empty();
            cell.fg_color = Some(color);
            self.grid.set_cell(row, col, cell);
        }

        let new_style = self.get_cell_style(row, col);
        let action = EditAction::SetStyle { row, col, old_style, new_style };
        self.undo_redo.undo_stack.push(action);
        self.undo_redo.redo_stack.clear();
    }

    /// Set font style for a cell
    pub fn set_cell_font_style(&mut self, row: usize, col: usize, bold: bool, italic: bool) {
        let old_style = self.get_cell_style(row, col);

        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            cell.font_bold = bold;
            cell.font_italic = italic;
        } else {
            let mut cell = Cell::empty();
            cell.font_bold = bold;
            cell.font_italic = italic;
            self.grid.set_cell(row, col, cell);
        }

        let new_style = self.get_cell_style(row, col);
        let action = EditAction::SetStyle { row, col, old_style, new_style };
        self.undo_redo.undo_stack.push(action);
        self.undo_redo.redo_stack.clear();
    }

    /// Clear background color for a cell
    pub fn clear_cell_bg_color(&mut self, row: usize, col: usize) {
        let old_style = self.get_cell_style(row, col);

        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            cell.bg_color = None;
        }

        let new_style = self.get_cell_style(row, col);
        let action = EditAction::SetStyle { row, col, old_style, new_style };
        self.undo_redo.undo_stack.push(action);
        self.undo_redo.redo_stack.clear();
    }

    /// Clear foreground color for a cell
    pub fn clear_cell_fg_color(&mut self, row: usize, col: usize) {
        let old_style = self.get_cell_style(row, col);

        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            cell.fg_color = None;
        }

        let new_style = self.get_cell_style(row, col);
        let action = EditAction::SetStyle { row, col, old_style, new_style };
        self.undo_redo.undo_stack.push(action);
        self.undo_redo.redo_stack.clear();
    }

    /// Set cell style (background, foreground, font) in one call
    pub fn set_cell_style(
        &mut self,
        row: usize,
        col: usize,
        bg_color: Option<u32>,
        fg_color: Option<u32>,
        bold: bool,
        italic: bool,
    ) {
        let old_style = self.get_cell_style(row, col);

        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            cell.bg_color = bg_color;
            cell.fg_color = fg_color;
            cell.font_bold = bold;
            cell.font_italic = italic;
        } else {
            let mut cell = Cell::empty();
            cell.bg_color = bg_color;
            cell.fg_color = fg_color;
            cell.font_bold = bold;
            cell.font_italic = italic;
            self.grid.set_cell(row, col, cell);
        }

        let new_style = self.get_cell_style(row, col);
        let action = EditAction::SetStyle { row, col, old_style, new_style };
        self.undo_redo.undo_stack.push(action);
        self.undo_redo.redo_stack.clear();
    }

    /// Set custom border for a cell (top, right, bottom, or left)
    /// side: 0=top, 1=right, 2=bottom, 3=left
    pub fn set_cell_border(&mut self, row: usize, col: usize, side: u8, color: u32, width: f32) {
        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            let border = Some(CellBorder { color, width });
            match side {
                0 => cell.border_top = border,
                1 => cell.border_right = border,
                2 => cell.border_bottom = border,
                3 => cell.border_left = border,
                _ => {}
            }
        } else {
            // Create cell if it doesn't exist
            let mut cell = Cell::empty();
            let border = Some(CellBorder { color, width });
            match side {
                0 => cell.border_top = border,
                1 => cell.border_right = border,
                2 => cell.border_bottom = border,
                3 => cell.border_left = border,
                _ => {}
            }
            self.grid.set_cell(row, col, cell);
        }
    }

    /// Set all borders for a cell at once
    pub fn set_cell_borders(&mut self, row: usize, col: usize, color: u32, width: f32) {
        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            let border = Some(CellBorder { color, width });
            cell.border_top = border.clone();
            cell.border_right = border.clone();
            cell.border_bottom = border.clone();
            cell.border_left = border;
        } else {
            let mut cell = Cell::empty();
            let border = Some(CellBorder { color, width });
            cell.border_top = border.clone();
            cell.border_right = border.clone();
            cell.border_bottom = border.clone();
            cell.border_left = border;
            self.grid.set_cell(row, col, cell);
        }
    }

    /// Clear border for a cell side
    /// side: 0=top, 1=right, 2=bottom, 3=left, 4=all
    pub fn clear_cell_border(&mut self, row: usize, col: usize, side: u8) {
        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            match side {
                0 => cell.border_top = None,
                1 => cell.border_right = None,
                2 => cell.border_bottom = None,
                3 => cell.border_left = None,
                4 => {
                    // Clear all borders
                    cell.border_top = None;
                    cell.border_right = None;
                    cell.border_bottom = None;
                    cell.border_left = None;
                }
                _ => {}
            }
        }
    }

    // ========== Column Grouping API ==========

    /// Add a column group for multi-level headers
    /// @param label - Group label text
    /// @param start_col - First column in group (0-indexed)
    /// @param end_col - Last column in group (0-indexed, inclusive)
    /// @param level - Header level (0 = top level, 1 = second level, etc.)
    pub fn add_column_group(&mut self, label: String, start_col: usize, end_col: usize, level: usize) {
        self.grid.add_column_group(label, start_col, end_col, level);
    }

    /// Clear all column groups (revert to single-level headers)
    pub fn clear_column_groups(&mut self) {
        self.grid.clear_column_groups();
    }

    /// Set the height of each header row (default: 30px)
    pub fn set_header_row_height(&mut self, height: f32) {
        self.grid.set_header_row_height(height);
    }

    /// Get the current number of header levels
    pub fn get_header_levels(&self) -> usize {
        self.grid.header_levels
    }

    /// Get total header height
    pub fn get_header_height(&self) -> f32 {
        self.grid.col_header_height
    }

    // ========== Column Validation API ==========

    /// Set validation pattern for a column
    /// @param col - Column index (0-based)
    /// @param pattern - JavaScript regex pattern (e.g., "^[0-9]+$" for numbers only)
    /// @param message - Error message to display when validation fails
    pub fn set_column_validation(&mut self, col: usize, pattern: String, message: String) {
        self.grid.set_column_validation(col, pattern, message);
    }

    /// Clear validation pattern for a column
    pub fn clear_column_validation(&mut self, col: usize) {
        self.grid.clear_column_validation(col);
    }

    /// Get validation pattern and message for a column
    /// Returns JSON string: {"pattern": "regex", "message": "error msg"} or empty string if no validation
    pub fn get_column_validation(&self, col: usize) -> String {
        if let Some((pattern, message)) = self.grid.get_column_validation(col) {
            return format!(r#"{{"pattern":"{}","message":"{}"}}"#,
                         pattern.replace("\\", "\\\\").replace("\"", "\\\""),
                         message.replace("\\", "\\\\").replace("\"", "\\\""));
        }
        String::new()
    }

    // ========== Column Editable Control API ==========

    /// Set whether a column is editable
    /// @param col - Column index (0-based)
    /// @param editable - true: editable, false: read-only
    pub fn set_column_editable(&mut self, col: usize, editable: bool) {
        self.grid.set_column_editable(col, editable);
    }

    /// Check if a column is editable
    pub fn is_column_editable(&self, col: usize) -> bool {
        self.grid.is_column_editable(col)
    }

    /// Get editable status for all columns as JSON array
    /// Returns: "[true, false, true, ...]"
    pub fn get_all_column_editable_status(&self) -> String {
        let status = self.grid.get_all_column_editable_status();
        format!("[{}]", status.iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(","))
    }

    /// Set column header name
    /// @param col - Column index (0-based)
    /// @param name - Header name to display
    pub fn set_column_name(&mut self, col: usize, name: &str) {
        if col < self.grid.column_configs.len() {
            self.grid.column_configs[col].display_name = name.to_string();
        }
    }

    /// Insert a row at the specified position
    pub fn insert_row(&mut self, at_index: usize) {
        // Record action for undo (insert is opposite of delete, so we store as DeleteRow)
        let action = EditAction::DeleteRow {
            index: at_index,
            cells: Vec::new(), // Empty row being inserted
        };
        self.undo_redo.undo_stack.push(action);
        self.undo_redo.redo_stack.clear();

        self.grid.insert_row(at_index);
        self.clear_selection();
        self.viewport.update_visible_range(&self.grid);
    }

    /// Delete a row at the specified position
    pub fn delete_row(&mut self, index: usize) {
        // Save cells before deletion for undo
        let cells = self.grid.get_row_cells(index);

        let action = EditAction::InsertRow {
            index,
            cells,
        };
        self.undo_redo.undo_stack.push(action);
        self.undo_redo.redo_stack.clear();

        self.grid.delete_row(index);
        self.clear_selection();
        self.viewport.update_visible_range(&self.grid);
    }

    /// Delete multiple rows at once
    /// @param indices - JSON array of row indices to delete, e.g., "[0, 2, 5]"
    pub fn delete_rows(&mut self, indices_json: String) -> Result<(), JsValue> {
        // Parse indices
        let indices: Vec<usize> = serde_json::from_str(&indices_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse indices: {}", e)))?;

        if indices.is_empty() {
            return Ok(());
        }

        // Sort indices in descending order to delete from bottom to top
        // This prevents index shifting issues
        let mut sorted_indices = indices.clone();
        sorted_indices.sort_unstable();
        sorted_indices.reverse();

        // Save all rows for undo
        let mut deleted_rows = Vec::new();
        for &index in &sorted_indices {
            if index < self.grid.row_count() {
                let cells = self.grid.get_row_cells(index);
                deleted_rows.push((index, cells));
            }
        }

        // Record undo action
        let action = EditAction::DeleteRows {
            rows: deleted_rows.clone(),
        };
        self.undo_redo.undo_stack.push(action);
        self.undo_redo.redo_stack.clear();

        // Delete rows from bottom to top
        for &index in &sorted_indices {
            if index < self.grid.row_count() {
                self.grid.delete_row(index);
            }
        }

        self.clear_selection();
        self.viewport.update_visible_range(&self.grid);

        Ok(())
    }

    /// Get unique row indices from selected cells
    /// Returns JSON array of row indices, e.g., "[0, 2, 5]"
    pub fn get_selected_row_indices(&self) -> String {
        let mut rows: Vec<usize> = self.selection.selected_cells
            .iter()
            .map(|(row, _)| *row)
            .collect();

        // Remove duplicates and sort
        rows.sort_unstable();
        rows.dedup();

        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".to_string())
    }

    /// Insert a column at the specified position
    pub fn insert_column(&mut self, at_index: usize) {
        // Record action for undo (insert is opposite of delete, so we store as DeleteColumn)
        let action = EditAction::DeleteColumn {
            index: at_index,
            cells: Vec::new(), // Empty column being inserted
        };
        self.undo_redo.undo_stack.push(action);
        self.undo_redo.redo_stack.clear();

        self.grid.insert_column(at_index);
        self.clear_selection();
        self.viewport.update_visible_range(&self.grid);
    }

    /// Delete a column at the specified position
    pub fn delete_column(&mut self, index: usize) {
        // Save cells before deletion for undo
        let cells = self.grid.get_column_cells(index);

        let action = EditAction::InsertColumn {
            index,
            cells,
        };
        self.undo_redo.undo_stack.push(action);
        self.undo_redo.redo_stack.clear();

        self.grid.delete_column(index);
        self.clear_selection();
        self.viewport.update_visible_range(&self.grid);
    }

    /// Delete all empty rows (rows with no non-empty cells)
    pub fn delete_empty_rows(&mut self) -> usize {
        let mut rows_to_delete = Vec::new();

        // Find all empty rows
        for row in 0..self.grid.row_count() {
            let mut is_empty = true;
            for col in 0..self.grid.col_count() {
                if !self.grid.get_value(row, col).is_empty() {
                    is_empty = false;
                    break;
                }
            }
            if is_empty {
                rows_to_delete.push(row);
            }
        }

        // Delete rows from bottom to top to maintain indices
        let count = rows_to_delete.len();
        for row in rows_to_delete.iter().rev() {
            self.delete_row(*row);
        }

        self.clear_selection();
        self.viewport.update_visible_range(&self.grid);
        count
    }

    /// Check if a row is empty (all cells are empty)
    pub fn is_row_empty(&self, row: usize) -> bool {
        for col in 0..self.grid.col_count() {
            if !self.grid.get_value(row, col).is_empty() {
                return false;
            }
        }
        true
    }

    /// Find all modified (edited) cells
    pub fn find_modified_cells(&mut self) -> usize {
        self.search.search_results.clear();
        self.search.current_search_index = None;

        for row in 0..self.grid.row_count() {
            for col in 0..self.grid.col_count() {
                if let Some(cell) = self.grid.get_cell(row, col) {
                    if cell.modified {
                        self.search.search_results.push((row, col));
                    }
                }
            }
        }

        if !self.search.search_results.is_empty() {
            self.search.current_search_index = Some(0);
            let (row, col) = self.search.search_results[0];
            self.select_single_cell(row, col);
            self.ensure_cell_visible(row, col);
        }

        self.search.search_results.len()
    }

    /// Clear modified flags from all cells
    pub fn clear_all_modified_flags(&mut self) {
        for row in 0..self.grid.row_count() {
            for col in 0..self.grid.col_count() {
                if let Some(cell) = self.grid.get_cell_mut(row, col) {
                    cell.modified = false;
                }
            }
        }
    }

    /// Check if a cell is modified
    pub fn is_cell_modified(&self, row: usize, col: usize) -> bool {
        if let Some(cell) = self.grid.get_cell(row, col) {
            cell.modified
        } else {
            false
        }
    }

    /// Get count of modified cells
    pub fn get_modified_cell_count(&self) -> usize {
        let mut count = 0;
        for row in 0..self.grid.row_count() {
            for col in 0..self.grid.col_count() {
                if let Some(cell) = self.grid.get_cell(row, col) {
                    if cell.modified {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// Search for text in grid cells (case-insensitive by default)
    pub fn search_text(&mut self, query: String) -> usize {
        self.search_text_with_options(query, false, false)
    }

    /// Search for text with options
    pub fn search_text_with_options(&mut self, query: String, case_sensitive: bool, whole_word: bool) -> usize {
        self.search.search_case_sensitive = case_sensitive;
        self.search.search_whole_word = whole_word;
        self.search.search_query = if case_sensitive { query.clone() } else { query.to_lowercase() };
        self.search.search_results.clear();
        self.search.current_search_index = None;

        if query.is_empty() {
            return 0;
        }

        // Search through all cells
        for row in 0..self.grid.row_count() {
            for col in 0..self.grid.col_count() {
                let cell_text = self.grid.get_value_string(row, col);
                let search_text = if case_sensitive { cell_text.clone() } else { cell_text.to_lowercase() };

                let is_match = if whole_word {
                    // Whole word matching: split by whitespace and check for exact match
                    search_text.split_whitespace().any(|word| word == self.search.search_query)
                } else {
                    // Substring matching
                    search_text.contains(&self.search.search_query)
                };

                if is_match {
                    self.search.search_results.push((row, col));
                }
            }
        }

        if !self.search.search_results.is_empty() {
            self.search.current_search_index = Some(0);
            let (row, col) = self.search.search_results[0];
            self.select_single_cell(row, col);
            self.ensure_cell_visible(row, col);
        }

        self.search.search_results.len()
    }

    /// Move to next search result
    pub fn search_next(&mut self) -> bool {
        if self.search.search_results.is_empty() {
            return false;
        }

        if let Some(current_idx) = self.search.current_search_index {
            let next_idx = (current_idx + 1) % self.search.search_results.len();
            self.search.current_search_index = Some(next_idx);

            let (row, col) = self.search.search_results[next_idx];
            self.select_single_cell(row, col);
            self.ensure_cell_visible(row, col);
            true
        } else {
            false
        }
    }

    /// Move to previous search result
    pub fn search_prev(&mut self) -> bool {
        if self.search.search_results.is_empty() {
            return false;
        }

        if let Some(current_idx) = self.search.current_search_index {
            let prev_idx = if current_idx == 0 {
                self.search.search_results.len() - 1
            } else {
                current_idx - 1
            };
            self.search.current_search_index = Some(prev_idx);

            let (row, col) = self.search.search_results[prev_idx];
            self.select_single_cell(row, col);
            self.ensure_cell_visible(row, col);
            true
        } else {
            false
        }
    }

    /// Search using regular expression
    pub fn search_regex(&mut self, pattern: String, case_sensitive: bool) -> Result<usize, String> {
        use regex::RegexBuilder;

        // Build regex with case sensitivity option
        let regex = match RegexBuilder::new(&pattern)
            .case_insensitive(!case_sensitive)
            .build()
        {
            Ok(re) => re,
            Err(e) => return Err(format!("Invalid regex pattern: {}", e)),
        };

        self.search.search_query = pattern;
        self.search.search_case_sensitive = case_sensitive;
        self.search.search_whole_word = false; // Not applicable for regex
        self.search.search_results.clear();
        self.search.current_search_index = None;

        // Search through all cells
        for row in 0..self.grid.row_count() {
            for col in 0..self.grid.col_count() {
                let cell_text = self.grid.get_value_string(row, col);

                if regex.is_match(&cell_text) {
                    self.search.search_results.push((row, col));
                }
            }
        }

        if !self.search.search_results.is_empty() {
            self.search.current_search_index = Some(0);
            let (row, col) = self.search.search_results[0];
            self.select_single_cell(row, col);
            self.ensure_cell_visible(row, col);
        }

        Ok(self.search.search_results.len())
    }

    /// Validate regex pattern without performing search
    pub fn validate_regex_pattern(&self, pattern: String) -> bool {
        use regex::Regex;
        Regex::new(&pattern).is_ok()
    }

    /// Clear search results
    pub fn clear_search(&mut self) {
        self.search.search_query.clear();
        self.search.search_results.clear();
        self.search.current_search_index = None;
    }

    /// Get search result count
    pub fn get_search_result_count(&self) -> usize {
        self.search.search_results.len()
    }

    /// Get current search index (1-based for display)
    pub fn get_current_search_index(&self) -> i32 {
        if let Some(idx) = self.search.current_search_index {
            (idx + 1) as i32
        } else {
            -1
        }
    }

    /// Check if a cell is a search result
    pub fn is_search_result(&self, row: usize, col: usize) -> bool {
        self.search.search_results.contains(&(row, col))
    }

    /// Check if a cell is the current (active) search result
    pub fn is_current_search_result(&self, row: usize, col: usize) -> bool {
        if let Some(idx) = self.search.current_search_index {
            if idx < self.search.search_results.len() {
                self.search.search_results[idx] == (row, col)
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Replace current search result with new text
    pub fn replace_current(&mut self, replacement: String) -> bool {
        if let Some(idx) = self.search.current_search_index {
            if idx < self.search.search_results.len() {
                let (row, col) = self.search.search_results[idx];

                // Parse replacement as number if possible
                if let Ok(num) = replacement.parse::<f64>() {
                    self.grid.set_value(row, col, CellValue::Number(num));
                } else {
                    self.grid.set_value(row, col, CellValue::Text(replacement));
                }

                // Move to next search result (or wrap around)
                self.search_next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Replace all search results with new text
    pub fn replace_all(&mut self, replacement: String) -> usize {
        let count = self.search.search_results.len();

        // Replace all matching cells
        for (row, col) in &self.search.search_results {
            // Parse replacement as number if possible
            if let Ok(num) = replacement.parse::<f64>() {
                self.grid.set_value(*row, *col, CellValue::Number(num));
            } else {
                self.grid.set_value(*row, *col, CellValue::Text(replacement.clone()));
            }
        }

        // Clear search results after replacing all
        self.clear_search();
        count
    }

    /// Replace in selection only
    pub fn replace_in_selection(&mut self, search: String, replacement: String, case_sensitive: bool) -> usize {
        let mut count = 0;
        let search_str = if case_sensitive { search.clone() } else { search.to_lowercase() };

        // Get list of selected cells
        let selected: Vec<(usize, usize)> = self.selection.selected_cells.iter().cloned().collect();

        for (row, col) in selected {
            let cell_text = self.grid.get_value_string(row, col);
            let search_text = if case_sensitive { cell_text.clone() } else { cell_text.to_lowercase() };

            if search_text.contains(&search_str) {
                // Parse replacement as number if possible
                if let Ok(num) = replacement.parse::<f64>() {
                    self.grid.set_value(row, col, CellValue::Number(num));
                } else {
                    self.grid.set_value(row, col, CellValue::Text(replacement.clone()));
                }
                count += 1;
            }
        }

        count
    }

    /// Sort by column
    pub fn sort_by_column(&mut self, col: usize, ascending: bool) {
        self.grid.sort_by_column(col, ascending);
        self.clear_selection();
        self.viewport.update_visible_range(&self.grid);
    }

    /// Toggle sort on column (click column header)
    pub fn toggle_column_sort(&mut self, col: usize) {
        // If already sorting by this column, toggle direction
        if self.grid.sort_column == Some(col) {
            self.sort_by_column(col, !self.grid.sort_ascending);
        } else {
            // Otherwise, sort ascending
            self.sort_by_column(col, true);
        }
    }

    /// Get sort state for a column as JSON object
    /// Returns: "{\"is_sorted\": true/false, \"is_ascending\": true/false}"
    pub fn get_column_sort_state(&self, col: usize) -> String {
        let (is_sorted, is_ascending) = if self.grid.sort_column == Some(col) {
            (true, self.grid.sort_ascending)
        } else {
            (false, true)
        };
        format!(r#"{{"is_sorted":{},"is_ascending":{}}}"#, is_sorted, is_ascending)
    }

    /// Add column to multi-column sort (for Shift+Click)
    pub fn add_multi_column_sort(&mut self, col: usize, ascending: bool) {
        self.grid.add_sort_column(col, ascending);
        self.clear_selection();
        self.viewport.update_visible_range(&self.grid);
    }

    /// Toggle column in multi-column sort
    pub fn toggle_multi_column_sort(&mut self, col: usize) {
        // Check if column is already in sort list
        let existing = self.grid.sort_columns.iter()
            .find(|(c, _)| *c == col)
            .map(|(_, asc)| *asc);

        match existing {
            Some(true) => {
                // Currently ascending, switch to descending
                self.add_multi_column_sort(col, false);
            }
            Some(false) => {
                // Currently descending, remove from sort
                self.grid.sort_columns.retain(|(c, _)| *c != col);
                if self.grid.sort_columns.is_empty() {
                    self.grid.sort_column = None;
                } else {
                    self.grid.sort_by_multiple_columns();
                }
                self.viewport.update_visible_range(&self.grid);
            }
            None => {
                // Not in sort list, add as ascending
                self.add_multi_column_sort(col, true);
            }
        }
    }

    /// Clear multi-column sort
    pub fn clear_multi_column_sort(&mut self) {
        self.grid.clear_multi_column_sort();
        self.viewport.update_visible_range(&self.grid);
    }

    /// Get multi-column sort state as JSON array of [col, ascending] pairs
    /// Returns: "[[col1, 1], [col2, 0], ...]" where 1=ascending, 0=descending
    pub fn get_multi_column_sort_state(&self) -> String {
        let state: Vec<Vec<u32>> = self.grid.sort_columns.iter()
            .map(|(col, asc)| vec![*col as u32, if *asc { 1 } else { 0 }])
            .collect();
        serde_json::to_string(&state).unwrap_or_else(|_| "[]".to_string())
    }

    /// Check if a column is in multi-column sort
    /// Returns JSON: "{\"is_sorted\": bool, \"is_ascending\": bool, \"sort_priority\": number}"
    pub fn get_column_multi_sort_state(&self, col: usize) -> String {
        if let Some((priority, (_, ascending))) = self.grid.sort_columns.iter()
            .enumerate()
            .find(|(_, (c, _))| *c == col)
        {
            format!(r#"{{"is_sorted":true,"is_ascending":{},"sort_priority":{}}}"#, ascending, priority)
        } else {
            r#"{"is_sorted":false,"is_ascending":true,"sort_priority":-1}"#.to_string()
        }
    }

    /// Freeze first N rows
    pub fn freeze_rows(&mut self, count: usize) {
        self.grid.frozen_rows = count.min(self.grid.row_count());
    }

    /// Freeze first N columns
    pub fn freeze_cols(&mut self, count: usize) {
        self.grid.frozen_cols = count.min(self.grid.col_count());
    }

    /// Get frozen row count
    pub fn get_frozen_rows(&self) -> usize {
        self.grid.frozen_rows
    }

    /// Set frozen row count
    pub fn set_frozen_rows(&mut self, count: usize) {
        self.grid.frozen_rows = count;
    }

    /// Get frozen column count
    pub fn get_frozen_cols(&self) -> usize {
        self.grid.frozen_cols
    }

    /// Set frozen column count
    pub fn set_frozen_cols(&mut self, count: usize) {
        self.grid.frozen_cols = count;
    }

    /// Undo last edit action
    pub fn undo(&mut self) -> bool {
        if let Some(action) = self.undo_redo.undo_stack.pop() {
            match &action {
                EditAction::SetValue { row, col, old_value, new_value: _ } => {
                    // Restore old value without recording undo
                    self.grid.set_value(*row, *col, old_value.clone());
                }
                EditAction::InsertRow { index, cells: _ } => {
                    // Undo insert by deleting the row
                    self.grid.delete_row(*index);
                    self.viewport.update_visible_range(&self.grid);
                }
                EditAction::DeleteRow { index, cells } => {
                    // Undo delete by inserting the row back
                    self.grid.insert_row(*index);
                    self.grid.restore_row_cells(*index, cells);
                    self.viewport.update_visible_range(&self.grid);
                }
                EditAction::InsertColumn { index, cells: _ } => {
                    // Undo insert by deleting the column
                    self.grid.delete_column(*index);
                    self.viewport.update_visible_range(&self.grid);
                }
                EditAction::DeleteColumn { index, cells } => {
                    // Undo delete by inserting the column back
                    self.grid.insert_column(*index);
                    self.grid.restore_column_cells(*index, cells);
                    self.viewport.update_visible_range(&self.grid);
                }
                EditAction::DeleteRows { rows } => {
                    // Undo bulk delete by inserting rows back in reverse order
                    for (index, cells) in rows.iter() {
                        self.grid.insert_row(*index);
                        self.grid.restore_row_cells(*index, cells);
                    }
                    self.viewport.update_visible_range(&self.grid);
                }
                EditAction::ClearCells { cells } => {
                    // Restore all cleared cell values
                    for (row, col, old_value) in cells.iter() {
                        self.grid.set_value(*row, *col, old_value.clone());
                    }
                }
                EditAction::SetMultipleCells { cells } => {
                    // Restore all old cell values
                    for (row, col, old_value, _new_value) in cells.iter() {
                        self.grid.set_value(*row, *col, old_value.clone());
                    }
                }
                EditAction::SetStyle { row, col, old_style, new_style: _ } => {
                    // Restore old style
                    if let Some(cell) = self.grid.get_cell_mut(*row, *col) {
                        cell.bg_color = old_style.bg_color;
                        cell.fg_color = old_style.fg_color;
                        cell.font_bold = old_style.font_bold;
                        cell.font_italic = old_style.font_italic;
                    }
                }
            }

            // Move action to redo stack
            self.undo_redo.redo_stack.push(action);
            true
        } else {
            false
        }
    }

    /// Redo last undone action
    pub fn redo(&mut self) -> bool {
        if let Some(action) = self.undo_redo.redo_stack.pop() {
            match &action {
                EditAction::SetValue { row, col, old_value: _, new_value } => {
                    // Re-apply new value without recording undo
                    self.grid.set_value(*row, *col, new_value.clone());
                }
                EditAction::InsertRow { index, cells } => {
                    // Redo insert
                    self.grid.insert_row(*index);
                    self.grid.restore_row_cells(*index, cells);
                    self.viewport.update_visible_range(&self.grid);
                }
                EditAction::DeleteRow { index, cells: _ } => {
                    // Redo delete
                    self.grid.delete_row(*index);
                    self.viewport.update_visible_range(&self.grid);
                }
                EditAction::InsertColumn { index, cells } => {
                    // Redo insert
                    self.grid.insert_column(*index);
                    self.grid.restore_column_cells(*index, cells);
                    self.viewport.update_visible_range(&self.grid);
                }
                EditAction::DeleteColumn { index, cells: _ } => {
                    // Redo delete
                    self.grid.delete_column(*index);
                    self.viewport.update_visible_range(&self.grid);
                }
                EditAction::DeleteRows { rows } => {
                    // Redo bulk delete from bottom to top to avoid index shifting
                    let mut sorted_indices: Vec<usize> = rows.iter().map(|(idx, _)| *idx).collect();
                    sorted_indices.sort_unstable();
                    sorted_indices.reverse();
                    for index in sorted_indices {
                        self.grid.delete_row(index);
                    }
                    self.viewport.update_visible_range(&self.grid);
                }
                EditAction::ClearCells { cells } => {
                    // Re-clear all cells
                    for (row, col, _old_value) in cells.iter() {
                        self.grid.set_value(*row, *col, CellValue::Empty);
                    }
                }
                EditAction::SetMultipleCells { cells } => {
                    // Re-apply all new cell values
                    for (row, col, _old_value, new_value) in cells.iter() {
                        self.grid.set_value(*row, *col, new_value.clone());
                    }
                }
                EditAction::SetStyle { row, col, old_style: _, new_style } => {
                    // Re-apply new style
                    if let Some(cell) = self.grid.get_cell_mut(*row, *col) {
                        cell.bg_color = new_style.bg_color;
                        cell.fg_color = new_style.fg_color;
                        cell.font_bold = new_style.font_bold;
                        cell.font_italic = new_style.font_italic;
                    }
                }
            }

            // Move action back to undo stack
            self.undo_redo.undo_stack.push(action);
            true
        } else {
            false
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_redo.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.undo_redo.redo_stack.is_empty()
    }

    /// Get undo stack size
    pub fn get_undo_count(&self) -> usize {
        self.undo_redo.undo_stack.len()
    }

    /// Get redo stack size
    pub fn get_redo_count(&self) -> usize {
        self.undo_redo.redo_stack.len()
    }

    /// Auto-fit column width to content
    pub fn auto_fit_column(&mut self, col: usize) {
        if col >= self.grid.col_count() {
            return;
        }

        let padding = 20.0; // Padding on both sides
        let min_width = 50.0;
        let max_width = 400.0_f32;

        // Measure all cells in this column
        let mut max_text_width = 0.0_f32;

        for row in 0..self.grid.row_count() {
            let text = self.grid.get_value_string(row, col);
            if !text.is_empty() {
                let text_width = self.text_renderer.measure_text(&text);
                max_text_width = max_text_width.max(text_width);
            }
        }

        // Also measure column header
        let header_text = Grid::get_col_name(col);
        let header_width = self.text_renderer.measure_text(&header_text);
        max_text_width = max_text_width.max(header_width);

        // Calculate optimal width with padding
        let optimal_width = (max_text_width + padding).clamp(min_width, max_width);

        self.grid.set_col_width(col, optimal_width);
        self.viewport.update_visible_range(&self.grid);
    }

    /// Auto-fit all columns to content
    pub fn auto_fit_all_columns(&mut self) {
        for col in 0..self.grid.col_count() {
            self.auto_fit_column(col);
        }
    }

    /// Set all columns to equal width
    pub fn set_all_columns_equal_width(&mut self, width: f32) {
        for col in 0..self.grid.col_count() {
            self.grid.set_col_width(col, width);
        }
        self.viewport.update_visible_range(&self.grid);
    }

    /// Filter column by text (case-insensitive contains)
    pub fn filter_column_by_text(&mut self, col: usize, text: String) {
        let filter_text = text.to_lowercase();
        self.grid.apply_column_filter(col, |value| {
            let cell_text = match value {
                CellValue::Text(t) => t.to_lowercase(),
                CellValue::Number(n) => n.to_string(),
                CellValue::Boolean(b) => b.to_string(),
                CellValue::Date(d) => d.to_lowercase(),
                CellValue::Empty => String::new(),
            };
            cell_text.contains(&filter_text)
        });
        self.viewport.update_visible_range(&self.grid);
    }

    /// Filter column by empty cells
    pub fn filter_column_show_non_empty(&mut self, col: usize) {
        self.grid.apply_column_filter(col, |value| {
            !matches!(value, CellValue::Empty)
        });
        self.viewport.update_visible_range(&self.grid);
    }

    /// Clear all column filters
    pub fn clear_column_filters(&mut self) {
        self.grid.clear_filters();
        self.viewport.update_visible_range(&self.grid);
    }

    /// Check if a row is filtered (hidden)
    pub fn is_row_filtered(&self, row: usize) -> bool {
        self.grid.is_row_filtered(row)
    }

    /// Get visible row count
    pub fn get_visible_row_count(&self) -> usize {
        self.grid.visible_row_count()
    }
}

/// Initialize the library
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"DataGrid5 initialized".into());
}

// Performance monitoring methods (outside wasm_bindgen)
impl DataGrid {
    /// Update FPS measurement
    fn update_fps(&mut self, current_time: f64) {
        if self.last_frame_time > 0.0 {
            let frame_time = current_time - self.last_frame_time;
            self.fps_samples.push(frame_time);

            // Keep only last 60 samples
            if self.fps_samples.len() > 60 {
                self.fps_samples.remove(0);
            }
        }

        self.last_frame_time = current_time;
        self.frame_count += 1;
    }

    /// Calculate current FPS
    fn calculate_fps(&self) -> f64 {
        if self.fps_samples.is_empty() {
            return 0.0;
        }

        let avg_frame_time: f64 = self.fps_samples.iter().sum::<f64>() / self.fps_samples.len() as f64;
        if avg_frame_time > 0.0 {
            1000.0 / avg_frame_time
        } else {
            0.0
        }
    }
}

#[wasm_bindgen]
impl DataGrid {
    /// Start performance benchmark (returns start time)
    pub fn benchmark_start(&self) -> f64 {
        web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0)
    }

    /// End performance benchmark and return elapsed time in ms
    pub fn benchmark_end(&self, start_time: f64) -> f64 {
        let end_time = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);
        end_time - start_time
    }

    /// Update FPS tracking (call this in render loop)
    pub fn update_performance_metrics(&mut self) {
        let current_time = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        self.update_fps(current_time);
    }

    /// Get current FPS
    pub fn get_fps(&self) -> f64 {
        self.calculate_fps()
    }

    /// Get total frame count
    pub fn get_frame_count(&self) -> u32 {
        self.frame_count
    }

    /// Get last render time in ms
    pub fn get_last_render_time(&self) -> f64 {
        self.render_time_ms
    }

    /// Reset performance metrics
    pub fn reset_performance_metrics(&mut self) {
        self.fps_samples.clear();
        self.last_frame_time = 0.0;
        self.frame_count = 0;
        self.render_time_ms = 0.0;
    }

    /// Run performance benchmark (render N frames and return average time)
    pub fn run_benchmark(&mut self, frame_count: u32) -> f64 {
        let start = self.benchmark_start();

        for _ in 0..frame_count {
            self.render();
        }

        let total_time = self.benchmark_end(start);
        total_time / frame_count as f64
    }

    /// Mark a specific cell as dirty (needs re-rendering)
    pub fn mark_cell_dirty(&mut self, row: usize, col: usize) {
        if row < self.grid.row_count() && col < self.grid.col_count() {
            self.dirty_cells.insert((row, col));
        }
    }

    /// Mark all cells as dirty (force full re-render)
    pub fn mark_all_dirty(&mut self) {
        self.needs_full_render = true;
        self.dirty_cells.clear();
    }

    /// Clear dirty cells (after rendering)
    pub fn clear_dirty_cells(&mut self) {
        self.dirty_cells.clear();
        self.needs_full_render = false;
    }

    /// Get count of dirty cells
    pub fn get_dirty_cell_count(&self) -> usize {
        if self.needs_full_render {
            self.grid.row_count() * self.grid.col_count()
        } else {
            self.dirty_cells.len()
        }
    }

    /// Check if full render is needed
    pub fn needs_full_render(&self) -> bool {
        self.needs_full_render
    }

    /// Optimize memory by reserving capacity for expected data size
    pub fn reserve_capacity(&mut self, expected_cells: usize) {
        // Reserve capacity in undo/redo stacks
        if self.undo_redo.undo_stack.capacity() < expected_cells {
            self.undo_redo.undo_stack.reserve(expected_cells);
        }
        if self.undo_redo.redo_stack.capacity() < expected_cells {
            self.undo_redo.redo_stack.reserve(expected_cells);
        }

        // Reserve capacity for dirty cells tracking
        if self.dirty_cells.capacity() < expected_cells {
            self.dirty_cells.reserve(expected_cells);
        }

        // Reserve capacity for selected cells
        if self.selection.selected_cells.capacity() < expected_cells / 10 {
            self.selection.selected_cells.reserve(expected_cells / 10);
        }

        // Reserve capacity for search results
        if self.search.search_results.capacity() < expected_cells / 100 {
            self.search.search_results.reserve(expected_cells / 100);
        }
    }

    /// Clear all non-essential cached data to free memory
    pub fn clear_caches(&mut self) {
        self.search.search_results.clear();
        self.search.search_results.shrink_to_fit();

        self.dirty_cells.clear();
        self.dirty_cells.shrink_to_fit();

        self.fps_samples.clear();
        self.fps_samples.shrink_to_fit();
    }

    /// Get estimated memory usage in bytes (approximate)
    pub fn get_memory_usage(&self) -> usize {
        let cell_count = self.grid.row_count() * self.grid.col_count();
        let selected_cells_mem = self.selection.selected_cells.len() * std::mem::size_of::<(usize, usize)>();
        let search_results_mem = self.search.search_results.len() * std::mem::size_of::<(usize, usize)>();
        let undo_stack_mem = self.undo_redo.undo_stack.capacity() * std::mem::size_of::<EditAction>();
        let redo_stack_mem = self.undo_redo.redo_stack.capacity() * std::mem::size_of::<EditAction>();
        let dirty_cells_mem = self.dirty_cells.len() * std::mem::size_of::<(usize, usize)>();

        // Base structure + hash maps + vectors
        let base_mem = std::mem::size_of::<Self>();
        let grid_mem = cell_count * 100; // Approximate per-cell overhead

        base_mem + grid_mem + selected_cells_mem + search_results_mem +
        undo_stack_mem + redo_stack_mem + dirty_cells_mem
    }

    /// Compact memory by removing unused allocations
    pub fn compact_memory(&mut self) {
        // Shrink vectors to fit actual data
        self.search.search_results.shrink_to_fit();
        self.undo_redo.undo_stack.shrink_to_fit();
        self.undo_redo.redo_stack.shrink_to_fit();
        self.fps_samples.shrink_to_fit();

        // Keep dirty_cells and selected_cells at reasonable capacity
        if self.dirty_cells.capacity() > self.dirty_cells.len() * 2 {
            self.dirty_cells.shrink_to_fit();
        }
        if self.selection.selected_cells.capacity() > self.selection.selected_cells.len() * 2 {
            self.selection.selected_cells.shrink_to_fit();
        }
    }

    // ============================================================================
    // Worker Thread Support for Background Data Processing
    // ============================================================================

    /// Export grid data as JSON for worker thread processing
    /// Returns JSON array: [{"row":0,"col":0,"value":"text","type":"text"}, ...]
    pub fn export_grid_data_json(&self) -> String {
        let mut data = Vec::new();

        for row in 0..self.grid.row_count() {
            for col in 0..self.grid.col_count() {
                if let Some(_cell) = self.grid.get_cell(row, col) {
                    let value = self.grid.get_value(row, col);
                    if !matches!(value, CellValue::Empty) {
                        let (value_str, type_str) = match value {
                            CellValue::Text(s) => (s.clone(), "text"),
                            CellValue::Number(n) => (n.to_string(), "number"),
                            CellValue::Boolean(b) => (b.to_string(), "boolean"),
                            CellValue::Date(d) => (d.clone(), "date"),
                            CellValue::Empty => continue,
                        };

                        data.push(format!(
                            r#"{{"row":{},"col":{},"value":"{}","type":"{}"}}"#,
                            row,
                            col,
                            value_str.replace("\"", "\\\""),
                            type_str
                        ));
                    }
                }
            }
        }

        format!("[{}]", data.join(","))
    }

    /// Export a specific range of data as JSON for worker processing
    /// Returns JSON array for the specified range
    pub fn export_range_json(&self, start_row: usize, end_row: usize, start_col: usize, end_col: usize) -> String {
        let mut data = Vec::new();

        let end_row = end_row.min(self.grid.row_count() - 1);
        let end_col = end_col.min(self.grid.col_count() - 1);

        for row in start_row..=end_row {
            for col in start_col..=end_col {
                if let Some(_cell) = self.grid.get_cell(row, col) {
                    let value = self.grid.get_value(row, col);
                    if !matches!(value, CellValue::Empty) {
                        let (value_str, type_str) = match value {
                            CellValue::Text(s) => (s.clone(), "text"),
                            CellValue::Number(n) => (n.to_string(), "number"),
                            CellValue::Boolean(b) => (b.to_string(), "boolean"),
                            CellValue::Date(d) => (d.clone(), "date"),
                            CellValue::Empty => continue,
                        };

                        data.push(format!(
                            r#"{{"row":{},"col":{},"value":"{}","type":"{}"}}"#,
                            row,
                            col,
                            value_str.replace("\"", "\\\""),
                            type_str
                        ));
                    }
                }
            }
        }

        format!("[{}]", data.join(","))
    }

    /// Import processed data from worker thread
    /// Accepts JSON array: [{"row":0,"col":0,"value":"text","type":"text"}, ...]
    pub fn import_worker_result(&mut self, result_json: &str) -> Result<usize, JsValue> {
        let data: Vec<serde_json::Value> = serde_json::from_str(result_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid JSON: {}", e)))?;

        let mut updated_count = 0;

        for cell_data in data {
            let row = cell_data["row"].as_u64().unwrap_or(0) as usize;
            let col = cell_data["col"].as_u64().unwrap_or(0) as usize;

            if row >= self.grid.row_count() || col >= self.grid.col_count() {
                continue;
            }

            let cell_value = match cell_data["type"].as_str() {
                Some("text") => CellValue::Text(cell_data["value"].as_str().unwrap_or("").to_string()),
                Some("number") => {
                    let val = cell_data["value"].as_f64()
                        .or_else(|| cell_data["value"].as_str().and_then(|s| s.parse::<f64>().ok()))
                        .unwrap_or(0.0);
                    CellValue::Number(val)
                }
                Some("boolean") => {
                    let val = cell_data["value"].as_bool()
                        .or_else(|| cell_data["value"].as_str().map(|s| s == "true"))
                        .unwrap_or(false);
                    CellValue::Boolean(val)
                }
                Some("date") => CellValue::Date(cell_data["value"].as_str().unwrap_or("").to_string()),
                _ => CellValue::Empty,
            };

            self.grid.set_value(row, col, cell_value);
            self.dirty_cells.insert((row, col));
            updated_count += 1;
        }

        Ok(updated_count)
    }

    /// Get grid metadata for worker thread (dimensions, frozen areas, etc.)
    pub fn get_grid_metadata_json(&self) -> String {
        format!(
            r#"{{"rows":{},"cols":{},"frozen_rows":{},"frozen_cols":{}}}"#,
            self.grid.row_count(),
            self.grid.col_count(),
            self.grid.frozen_rows,
            self.grid.frozen_cols
        )
    }

    /// Prepare data for sorting in worker thread
    /// Returns JSON with data and sort configuration
    /// sort_columns_json: JSON array of column indices, e.g. "[0, 1]"
    /// ascending_json: JSON array of booleans, e.g. "[true, false]"
    pub fn prepare_sort_data(&self, sort_columns_json: &str, ascending_json: &str) -> Result<String, JsValue> {
        let sort_columns: Vec<usize> = serde_json::from_str(sort_columns_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid sort_columns JSON: {}", e)))?;
        let ascending: Vec<bool> = serde_json::from_str(ascending_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid ascending JSON: {}", e)))?;

        Ok(format!(
            r#"{{"data":{},"sort_columns":{},"ascending":{}}}"#,
            self.export_grid_data_json(),
            serde_json::to_string(&sort_columns).unwrap_or_else(|_| "[]".to_string()),
            serde_json::to_string(&ascending).unwrap_or_else(|_| "[]".to_string())
        ))
    }

    /// Apply sorted row indices from worker result
    /// Takes array of row indices representing the new order
    pub fn apply_sorted_indices(&mut self, indices_json: &str) -> Result<(), JsValue> {
        let indices: Vec<usize> = serde_json::from_str(indices_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid indices JSON: {}", e)))?;

        if indices.len() != self.grid.row_count() {
            return Err(JsValue::from_str(&format!(
                "Indices length {} does not match row count {}",
                indices.len(),
                self.grid.row_count()
            )));
        }

        // Create a copy of all rows
        let mut row_data: Vec<Vec<CellValue>> = Vec::new();
        for row in 0..self.grid.row_count() {
            let mut row_values = Vec::new();
            for col in 0..self.grid.col_count() {
                row_values.push(self.grid.get_value(row, col));
            }
            row_data.push(row_values);
        }

        // Reorder rows according to indices
        for (new_row, &old_row) in indices.iter().enumerate() {
            if old_row >= row_data.len() {
                continue;
            }
            for (col, value) in row_data[old_row].iter().enumerate() {
                self.grid.set_value(new_row, col, value.clone());
                self.dirty_cells.insert((new_row, col));
            }
        }

        Ok(())
    }
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

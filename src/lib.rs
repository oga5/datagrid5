mod core;
mod input;
mod renderer;

use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, KeyboardEvent, MouseEvent, WheelEvent};

use core::{Cell, CellValue, Grid, Viewport};
use input::{KeyboardHandler, MouseHandler, NavigationCommand};
use renderer::{TextRenderer, WebGLRenderer};

/// Action that can be undone/redone
#[derive(Clone)]
enum EditAction {
    SetValue {
        row: usize,
        col: usize,
        old_value: CellValue,
        new_value: CellValue,
    },
}

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
    is_editing: bool,
    editing_cell: Option<(usize, usize)>,
    // Resize state
    is_resizing: bool,
    resizing_column: Option<usize>,
    resizing_row: Option<usize>,
    resize_start_pos: f32,
    resize_start_size: f32,
    // Multi-selection state
    selected_cells: HashSet<(usize, usize)>,
    selection_anchor: Option<(usize, usize)>,
    // Search state
    search_query: String,
    search_results: Vec<(usize, usize)>,
    current_search_index: Option<usize>,
    search_case_sensitive: bool,
    search_whole_word: bool,
    // Undo/Redo state
    undo_stack: Vec<EditAction>,
    redo_stack: Vec<EditAction>,
}

#[wasm_bindgen]
impl DataGrid {
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

        let mut grid = Grid::new(rows, cols);
        grid.fill_sample_data();

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
            is_editing: false,
            editing_cell: None,
            is_resizing: false,
            resizing_column: None,
            resizing_row: None,
            resize_start_pos: 0.0,
            resize_start_size: 0.0,
            selected_cells: HashSet::new(),
            selection_anchor: None,
            search_query: String::new(),
            search_results: Vec::new(),
            current_search_index: None,
            search_case_sensitive: false,
            search_whole_word: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
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
            &self.search_results,
            self.current_search_index
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

        self.mouse_handler.mouse_down(x, y);

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
            } else if ctrl {
                // Ctrl+Click: Toggle selection
                self.toggle_cell_selection(row, col);
            } else {
                // Normal click: Single selection
                self.select_single_cell(row, col);
            }

            self.mouse_handler.select_cell(row, col);
            web_sys::console::log_1(&format!("Selected {} cells", self.selected_cells.len()).into());
        } else {
            // Clicked outside grid, clear selection
            if !ctrl {
                self.clear_selection();
            }
            self.mouse_handler.selected_cell = None;
        }
    }

    /// Handle mouse down event (legacy, for backward compatibility)
    pub fn handle_mouse_down(&mut self, event: MouseEvent) {
        self.handle_mouse_down_with_modifiers(event, false, false);
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

            // Record action for undo
            let action = EditAction::SetValue {
                row,
                col,
                old_value,
                new_value,
            };

            self.undo_stack.push(action);

            // Clear redo stack on new edit
            self.redo_stack.clear();
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
                    // Clear cell content
                    if let Some((row, col)) = current {
                        self.grid.set_value(row, col, CellValue::Empty);
                        web_sys::console::log_1(&format!("Cleared cell: ({}, {})", row, col).into());
                    }
                    None
                }
                NavigationCommand::Undo => {
                    if self.undo() {
                        web_sys::console::log_1(&"Undo action".into());
                    }
                    None
                }
                NavigationCommand::Redo => {
                    if self.redo() {
                        web_sys::console::log_1(&"Redo action".into());
                    }
                    None
                }
                NavigationCommand::Enter | NavigationCommand::Escape | NavigationCommand::Tab => {
                    // Future: handle edit mode, etc.
                    None
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
                    }
                    None
                }
                NavigationCommand::Redo => {
                    if self.redo() {
                        web_sys::console::log_1(&"Redo action".into());
                    }
                    None
                }
                NavigationCommand::Enter | NavigationCommand::Escape | NavigationCommand::Tab => {
                    None
                }
            };

            if let Some((new_row, new_col)) = new_selection {
                if let Some((prev_row, prev_col)) = current {
                    if let Some(cell) = self.grid.get_cell_mut(prev_row, prev_col) {
                        cell.selected = false;
                    }
                }

                self.mouse_handler.select_cell(new_row, new_col);
                if let Some(cell) = self.grid.get_cell_mut(new_row, new_col) {
                    cell.selected = true;
                } else {
                    let mut cell = Cell::default();
                    cell.selected = true;
                    self.grid.set_cell(new_row, new_col, cell);
                }

                self.ensure_cell_visible(new_row, new_col);
                web_sys::console::log_1(&format!("Navigated to cell: ({}, {})", new_row, new_col).into());

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

        // Check horizontal visibility
        if cell_x < scroll_x {
            scroll_x = cell_x;
        } else if cell_x + cell_width > scroll_x + self.viewport.canvas_width {
            scroll_x = cell_x + cell_width - self.viewport.canvas_width;
        }

        // Check vertical visibility
        if cell_y < scroll_y {
            scroll_y = cell_y;
        } else if cell_y + cell_height > scroll_y + self.viewport.canvas_height {
            scroll_y = cell_y + cell_height - self.viewport.canvas_height;
        }

        // Update scroll if changed
        if scroll_x != self.viewport.scroll_x || scroll_y != self.viewport.scroll_y {
            self.viewport.set_scroll(scroll_x, scroll_y, &self.grid);
            self.viewport.update_visible_range(&self.grid);
        }
    }

    /// Start editing a cell (called from JavaScript)
    pub fn start_edit(&mut self, row: usize, col: usize) -> bool {
        // Check if cell is valid
        if row >= self.grid.row_count() || col >= self.grid.col_count() {
            return false;
        }

        // Check if cell is editable
        if let Some(cell) = self.grid.get_cell(row, col) {
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

    /// Update cell value during editing
    pub fn update_cell_value(&mut self, row: usize, col: usize, value: String) {
        if self.is_editing && self.editing_cell == Some((row, col)) {
            self.set_cell_value(row, col, &value);
            web_sys::console::log_1(&format!("Updated cell ({}, {}) to: {}", row, col, value).into());
        }
    }

    /// Get cell position for editing (returns canvas coordinates)
    pub fn get_cell_edit_rect(&self, row: usize, col: usize) -> Vec<f32> {
        let x = self.grid.col_x_position(col) - self.viewport.scroll_x;
        let y = self.grid.row_y_position(row) - self.viewport.scroll_y;
        let width = self.grid.col_width(col);
        let height = self.grid.row_height(row);

        vec![x, y, width, height]
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

    /// Check if mouse is over a resize handle
    /// Returns: "col" for column resize, "row" for row resize, "none" otherwise
    pub fn check_resize_handle(&self, x: f32, y: f32) -> String {
        const RESIZE_HANDLE_WIDTH: f32 = 5.0;

        let grid_x = x + self.viewport.scroll_x;
        let grid_y = y + self.viewport.scroll_y;

        // Check column resize handles
        let mut col_x = 0.0;
        for col in 0..self.grid.col_count() {
            let width = self.grid.col_width(col);
            col_x += width;

            // Check if near right edge of column
            if (grid_x - col_x).abs() < RESIZE_HANDLE_WIDTH {
                return "col".to_string();
            }
        }

        // Check row resize handles
        let mut row_y = 0.0;
        for row in 0..self.grid.row_count() {
            let height = self.grid.row_height(row);
            row_y += height;

            // Check if near bottom edge of row
            if (grid_y - row_y).abs() < RESIZE_HANDLE_WIDTH {
                return "row".to_string();
            }
        }

        "none".to_string()
    }

    /// Start column or row resize
    pub fn start_resize(&mut self, x: f32, y: f32, resize_type: &str) -> bool {
        const RESIZE_HANDLE_WIDTH: f32 = 5.0;

        let grid_x = x + self.viewport.scroll_x;
        let grid_y = y + self.viewport.scroll_y;

        if resize_type == "col" {
            // Find which column to resize
            let mut col_x = 0.0;
            for col in 0..self.grid.col_count() {
                let width = self.grid.col_width(col);
                col_x += width;

                if (grid_x - col_x).abs() < RESIZE_HANDLE_WIDTH {
                    self.is_resizing = true;
                    self.resizing_column = Some(col);
                    self.resize_start_pos = x;
                    self.resize_start_size = width;
                    web_sys::console::log_1(&format!("Started resizing column {}", col).into());
                    return true;
                }
            }
        } else if resize_type == "row" {
            // Find which row to resize
            let mut row_y = 0.0;
            for row in 0..self.grid.row_count() {
                let height = self.grid.row_height(row);
                row_y += height;

                if (grid_y - row_y).abs() < RESIZE_HANDLE_WIDTH {
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
    pub fn update_resize(&mut self, x: f32, y: f32) {
        if !self.is_resizing {
            return;
        }

        if let Some(col) = self.resizing_column {
            let delta = x - self.resize_start_pos;
            let new_width = (self.resize_start_size + delta).max(30.0); // Minimum 30px
            self.grid.set_col_width(col, new_width);
        } else if let Some(row) = self.resizing_row {
            let delta = y - self.resize_start_pos;
            let new_height = (self.resize_start_size + delta).max(20.0); // Minimum 20px
            self.grid.set_row_height(row, new_height);
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

    /// Select a single cell (clears previous selection)
    fn select_single_cell(&mut self, row: usize, col: usize) {
        // Clear all previous selections
        self.clear_selection();

        // Add new selection
        self.selected_cells.insert((row, col));
        self.selection_anchor = Some((row, col));

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
        if self.selected_cells.contains(&(row, col)) {
            // Remove from selection
            self.selected_cells.remove(&(row, col));
            if let Some(cell) = self.grid.get_cell_mut(row, col) {
                cell.selected = false;
            }
        } else {
            // Add to selection
            self.selected_cells.insert((row, col));
            if let Some(cell) = self.grid.get_cell_mut(row, col) {
                cell.selected = true;
            } else {
                let mut cell = Cell::default();
                cell.selected = true;
                self.grid.set_cell(row, col, cell);
            }
        }

        // Update anchor
        if !self.selected_cells.is_empty() {
            self.selection_anchor = Some((row, col));
        }
    }

    /// Select range from anchor to target cell
    fn select_range(&mut self, target_row: usize, target_col: usize) {
        if let Some((anchor_row, anchor_col)) = self.selection_anchor {
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
                        self.selected_cells.insert((r, c));

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
        for (row, col) in &self.selected_cells {
            if let Some(cell) = self.grid.get_cell_mut(*row, *col) {
                cell.selected = false;
            }
        }
        self.selected_cells.clear();
    }

    /// Get selected cells as a list of [row, col] pairs
    pub fn get_selected_cells(&self) -> Vec<Vec<usize>> {
        self.selected_cells
            .iter()
            .map(|(row, col)| vec![*row, *col])
            .collect()
    }

    /// Get selection count
    pub fn get_selection_count(&self) -> usize {
        self.selected_cells.len()
    }

    /// Select all cells (Ctrl+A)
    pub fn select_all(&mut self) {
        self.clear_selection();

        for row in 0..self.grid.row_count() {
            for col in 0..self.grid.col_count() {
                self.selected_cells.insert((row, col));
                if let Some(cell) = self.grid.get_cell_mut(row, col) {
                    cell.selected = true;
                }
            }
        }

        // Set anchor to first cell
        self.selection_anchor = Some((0, 0));
    }

    /// Select entire row
    pub fn select_row(&mut self, row: usize) {
        if row >= self.grid.row_count() {
            return;
        }

        self.clear_selection();

        for col in 0..self.grid.col_count() {
            self.selected_cells.insert((row, col));
            if let Some(cell) = self.grid.get_cell_mut(row, col) {
                cell.selected = true;
            }
        }

        self.selection_anchor = Some((row, 0));
    }

    /// Select entire column
    pub fn select_col(&mut self, col: usize) {
        if col >= self.grid.col_count() {
            return;
        }

        self.clear_selection();

        for row in 0..self.grid.row_count() {
            self.selected_cells.insert((row, col));
            if let Some(cell) = self.grid.get_cell_mut(row, col) {
                cell.selected = true;
            }
        }

        self.selection_anchor = Some((0, col));
    }

    /// Copy selected cells to TSV (Tab-Separated Values) format
    /// Returns a string with cells separated by tabs and rows separated by newlines
    pub fn copy_selected_cells(&self) -> String {
        if self.selected_cells.is_empty() {
            return String::new();
        }

        // Sort selected cells by row, then by column
        let mut cells: Vec<(usize, usize)> = self.selected_cells.iter().copied().collect();
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
                if self.selected_cells.contains(&(row, col)) {
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

    /// Paste cells from TSV (Tab-Separated Values) format
    /// Pastes starting from the current focus cell
    pub fn paste_cells(&mut self, tsv_text: String) -> Result<(), String> {
        if tsv_text.is_empty() {
            return Ok(());
        }

        // Determine starting position (focus cell or first selected cell)
        let (start_row, start_col) = if let Some(anchor) = self.selection_anchor {
            anchor
        } else if !self.selected_cells.is_empty() {
            let mut cells: Vec<(usize, usize)> = self.selected_cells.iter().copied().collect();
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

        // Parse TSV and paste
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

                // Create cell value
                let cell_value = if value.is_empty() {
                    CellValue::Empty
                } else if let Ok(num) = value.parse::<f64>() {
                    CellValue::Number(num)
                } else if *value == "true" || *value == "false" {
                    CellValue::Boolean(*value == "true")
                } else {
                    CellValue::Text(value.to_string())
                };

                self.grid.set_value(target_row, target_col, cell_value);
            }
        }

        Ok(())
    }

    /// Set background color for a cell (RGBA as u32: 0xRRGGBBAA)
    pub fn set_cell_bg_color(&mut self, row: usize, col: usize, color: u32) {
        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            cell.bg_color = Some(color);
        } else {
            // Create cell if it doesn't exist
            let mut cell = Cell::empty();
            cell.bg_color = Some(color);
            self.grid.set_cell(row, col, cell);
        }
    }

    /// Set foreground (text) color for a cell (RGBA as u32: 0xRRGGBBAA)
    pub fn set_cell_fg_color(&mut self, row: usize, col: usize, color: u32) {
        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            cell.fg_color = Some(color);
        } else {
            let mut cell = Cell::empty();
            cell.fg_color = Some(color);
            self.grid.set_cell(row, col, cell);
        }
    }

    /// Set font style for a cell
    pub fn set_cell_font_style(&mut self, row: usize, col: usize, bold: bool, italic: bool) {
        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            cell.font_bold = bold;
            cell.font_italic = italic;
        } else {
            let mut cell = Cell::empty();
            cell.font_bold = bold;
            cell.font_italic = italic;
            self.grid.set_cell(row, col, cell);
        }
    }

    /// Clear background color for a cell
    pub fn clear_cell_bg_color(&mut self, row: usize, col: usize) {
        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            cell.bg_color = None;
        }
    }

    /// Clear foreground color for a cell
    pub fn clear_cell_fg_color(&mut self, row: usize, col: usize) {
        if let Some(cell) = self.grid.get_cell_mut(row, col) {
            cell.fg_color = None;
        }
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
    }

    /// Insert a row at the specified position
    pub fn insert_row(&mut self, at_index: usize) {
        self.grid.insert_row(at_index);
        self.clear_selection();
        self.viewport.update_visible_range(&self.grid);
    }

    /// Delete a row at the specified position
    pub fn delete_row(&mut self, index: usize) {
        self.grid.delete_row(index);
        self.clear_selection();
        self.viewport.update_visible_range(&self.grid);
    }

    /// Insert a column at the specified position
    pub fn insert_column(&mut self, at_index: usize) {
        self.grid.insert_column(at_index);
        self.clear_selection();
        self.viewport.update_visible_range(&self.grid);
    }

    /// Delete a column at the specified position
    pub fn delete_column(&mut self, index: usize) {
        self.grid.delete_column(index);
        self.clear_selection();
        self.viewport.update_visible_range(&self.grid);
    }

    /// Search for text in grid cells (case-insensitive by default)
    pub fn search_text(&mut self, query: String) -> usize {
        self.search_text_with_options(query, false, false)
    }

    /// Search for text with options
    pub fn search_text_with_options(&mut self, query: String, case_sensitive: bool, whole_word: bool) -> usize {
        self.search_case_sensitive = case_sensitive;
        self.search_whole_word = whole_word;
        self.search_query = if case_sensitive { query.clone() } else { query.to_lowercase() };
        self.search_results.clear();
        self.current_search_index = None;

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
                    search_text.split_whitespace().any(|word| word == self.search_query)
                } else {
                    // Substring matching
                    search_text.contains(&self.search_query)
                };

                if is_match {
                    self.search_results.push((row, col));
                }
            }
        }

        if !self.search_results.is_empty() {
            self.current_search_index = Some(0);
            let (row, col) = self.search_results[0];
            self.select_single_cell(row, col);
            self.ensure_cell_visible(row, col);
        }

        self.search_results.len()
    }

    /// Move to next search result
    pub fn search_next(&mut self) -> bool {
        if self.search_results.is_empty() {
            return false;
        }

        if let Some(current_idx) = self.current_search_index {
            let next_idx = (current_idx + 1) % self.search_results.len();
            self.current_search_index = Some(next_idx);

            let (row, col) = self.search_results[next_idx];
            self.select_single_cell(row, col);
            self.ensure_cell_visible(row, col);
            true
        } else {
            false
        }
    }

    /// Move to previous search result
    pub fn search_prev(&mut self) -> bool {
        if self.search_results.is_empty() {
            return false;
        }

        if let Some(current_idx) = self.current_search_index {
            let prev_idx = if current_idx == 0 {
                self.search_results.len() - 1
            } else {
                current_idx - 1
            };
            self.current_search_index = Some(prev_idx);

            let (row, col) = self.search_results[prev_idx];
            self.select_single_cell(row, col);
            self.ensure_cell_visible(row, col);
            true
        } else {
            false
        }
    }

    /// Clear search results
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.search_results.clear();
        self.current_search_index = None;
    }

    /// Get search result count
    pub fn get_search_result_count(&self) -> usize {
        self.search_results.len()
    }

    /// Get current search index (1-based for display)
    pub fn get_current_search_index(&self) -> i32 {
        if let Some(idx) = self.current_search_index {
            (idx + 1) as i32
        } else {
            -1
        }
    }

    /// Check if a cell is a search result
    pub fn is_search_result(&self, row: usize, col: usize) -> bool {
        self.search_results.contains(&(row, col))
    }

    /// Check if a cell is the current (active) search result
    pub fn is_current_search_result(&self, row: usize, col: usize) -> bool {
        if let Some(idx) = self.current_search_index {
            if idx < self.search_results.len() {
                self.search_results[idx] == (row, col)
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Replace current search result with new text
    pub fn replace_current(&mut self, replacement: String) -> bool {
        if let Some(idx) = self.current_search_index {
            if idx < self.search_results.len() {
                let (row, col) = self.search_results[idx];

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
        let count = self.search_results.len();

        // Replace all matching cells
        for (row, col) in &self.search_results {
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
        let selected: Vec<(usize, usize)> = self.selected_cells.iter().cloned().collect();

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

    /// Get sort state for a column (returns: (is_sorted, is_ascending))
    pub fn get_column_sort_state(&self, col: usize) -> (bool, bool) {
        if self.grid.sort_column == Some(col) {
            (true, self.grid.sort_ascending)
        } else {
            (false, true)
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

    /// Get frozen column count
    pub fn get_frozen_cols(&self) -> usize {
        self.grid.frozen_cols
    }

    /// Undo last edit action
    pub fn undo(&mut self) -> bool {
        if let Some(action) = self.undo_stack.pop() {
            match &action {
                EditAction::SetValue { row, col, old_value, new_value: _ } => {
                    // Restore old value without recording undo
                    self.grid.set_value(*row, *col, old_value.clone());
                }
            }

            // Move action to redo stack
            self.redo_stack.push(action);
            true
        } else {
            false
        }
    }

    /// Redo last undone action
    pub fn redo(&mut self) -> bool {
        if let Some(action) = self.redo_stack.pop() {
            match &action {
                EditAction::SetValue { row, col, old_value: _, new_value } => {
                    // Re-apply new value without recording undo
                    self.grid.set_value(*row, *col, new_value.clone());
                }
            }

            // Move action back to undo stack
            self.undo_stack.push(action);
            true
        } else {
            false
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get undo stack size
    pub fn get_undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get redo stack size
    pub fn get_redo_count(&self) -> usize {
        self.redo_stack.len()
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

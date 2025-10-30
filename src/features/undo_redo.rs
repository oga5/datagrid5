use crate::core::{cell::CellValue, Cell, Grid, Viewport};

/// Cell style information for undo/redo
#[derive(Clone, Debug)]
pub struct CellStyle {
    pub bg_color: Option<u32>,
    pub fg_color: Option<u32>,
    pub font_bold: bool,
    pub font_italic: bool,
}

/// Action that can be undone/redone
#[derive(Clone)]
pub enum EditAction {
    SetValue {
        row: usize,
        col: usize,
        old_value: CellValue,
        new_value: CellValue,
    },
    InsertRow {
        index: usize,
        // Store all cells in the row before deletion (for redo of delete)
        cells: Vec<(usize, Cell)>, // (col, cell)
    },
    DeleteRow {
        index: usize,
        // Store all cells in the row for undo
        cells: Vec<(usize, Cell)>, // (col, cell)
    },
    InsertColumn {
        index: usize,
        // Store all cells in the column before deletion (for redo of delete)
        cells: Vec<(usize, Cell)>, // (row, cell)
    },
    DeleteColumn {
        index: usize,
        // Store all cells in the column for undo
        cells: Vec<(usize, Cell)>, // (row, cell)
    },
    DeleteRows {
        // Store multiple rows for bulk deletion undo
        rows: Vec<(usize, Vec<(usize, Cell)>)>, // (row_index, cells)
    },
    ClearCells {
        // Store multiple cell values for bulk clear/delete undo
        cells: Vec<(usize, usize, CellValue)>, // (row, col, old_value)
    },
    SetMultipleCells {
        // Store multiple cell changes (e.g., for paste, bulk edit)
        cells: Vec<(usize, usize, CellValue, CellValue)>, // (row, col, old_value, new_value)
    },
    SetStyle {
        row: usize,
        col: usize,
        old_style: CellStyle,
        new_style: CellStyle,
    },
}

/// Undo/Redo functionality for DataGrid
pub struct UndoRedoState {
    pub undo_stack: Vec<EditAction>,
    pub redo_stack: Vec<EditAction>,
}

impl Default for UndoRedoState {
    fn default() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }
}

impl UndoRedoState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Perform undo operation
    pub fn undo(&mut self, grid: &mut Grid, viewport: &mut Viewport) -> bool {
        if let Some(action) = self.undo_stack.pop() {
            match &action {
                EditAction::SetValue { row, col, old_value, new_value: _ } => {
                    // Restore old value without recording undo
                    grid.set_value(*row, *col, old_value.clone());
                }
                EditAction::InsertRow { index, cells: _ } => {
                    // Undo insert by deleting the row
                    grid.delete_row(*index);
                    viewport.update_visible_range(grid);
                }
                EditAction::DeleteRow { index, cells } => {
                    // Undo delete by inserting the row back
                    grid.insert_row(*index);
                    grid.restore_row_cells(*index, cells);
                    viewport.update_visible_range(grid);
                }
                EditAction::InsertColumn { index, cells: _ } => {
                    // Undo insert by deleting the column
                    grid.delete_column(*index);
                    viewport.update_visible_range(grid);
                }
                EditAction::DeleteColumn { index, cells } => {
                    // Undo delete by inserting the column back
                    grid.insert_column(*index);
                    grid.restore_column_cells(*index, cells);
                    viewport.update_visible_range(grid);
                }
                EditAction::DeleteRows { rows } => {
                    // Undo bulk delete by inserting rows back in reverse order
                    for (index, cells) in rows.iter() {
                        grid.insert_row(*index);
                        grid.restore_row_cells(*index, cells);
                    }
                    viewport.update_visible_range(grid);
                }
                EditAction::ClearCells { cells } => {
                    // Restore all cleared cell values
                    for (row, col, old_value) in cells.iter() {
                        grid.set_value(*row, *col, old_value.clone());
                    }
                }
                EditAction::SetMultipleCells { cells } => {
                    // Restore all old cell values
                    for (row, col, old_value, _new_value) in cells.iter() {
                        grid.set_value(*row, *col, old_value.clone());
                    }
                }
                EditAction::SetStyle { row, col, old_style, new_style: _ } => {
                    // Restore old style
                    if let Some(cell) = grid.get_cell_mut(*row, *col) {
                        cell.bg_color = old_style.bg_color;
                        cell.fg_color = old_style.fg_color;
                        cell.font_bold = old_style.font_bold;
                        cell.font_italic = old_style.font_italic;
                    }
                }
            }

            // Move action to redo stack
            self.redo_stack.push(action);
            true
        } else {
            false
        }
    }

    /// Perform redo operation
    pub fn redo(&mut self, grid: &mut Grid, viewport: &mut Viewport) -> bool {
        if let Some(action) = self.redo_stack.pop() {
            match &action {
                EditAction::SetValue { row, col, old_value: _, new_value } => {
                    // Re-apply new value without recording undo
                    grid.set_value(*row, *col, new_value.clone());
                }
                EditAction::InsertRow { index, cells } => {
                    // Redo insert
                    grid.insert_row(*index);
                    grid.restore_row_cells(*index, cells);
                    viewport.update_visible_range(grid);
                }
                EditAction::DeleteRow { index, cells: _ } => {
                    // Redo delete
                    grid.delete_row(*index);
                    viewport.update_visible_range(grid);
                }
                EditAction::InsertColumn { index, cells } => {
                    // Redo insert
                    grid.insert_column(*index);
                    grid.restore_column_cells(*index, cells);
                    viewport.update_visible_range(grid);
                }
                EditAction::DeleteColumn { index, cells: _ } => {
                    // Redo delete
                    grid.delete_column(*index);
                    viewport.update_visible_range(grid);
                }
                EditAction::DeleteRows { rows } => {
                    // Redo bulk delete from bottom to top to avoid index shifting
                    let mut sorted_indices: Vec<usize> = rows.iter().map(|(idx, _)| *idx).collect();
                    sorted_indices.sort_unstable();
                    sorted_indices.reverse();
                    for index in sorted_indices {
                        grid.delete_row(index);
                    }
                    viewport.update_visible_range(grid);
                }
                EditAction::ClearCells { cells } => {
                    // Re-clear all cells
                    for (row, col, _old_value) in cells.iter() {
                        grid.set_value(*row, *col, CellValue::Empty);
                    }
                }
                EditAction::SetMultipleCells { cells } => {
                    // Re-apply all new cell values
                    for (row, col, _old_value, new_value) in cells.iter() {
                        grid.set_value(*row, *col, new_value.clone());
                    }
                }
                EditAction::SetStyle { row, col, old_style: _, new_style } => {
                    // Re-apply new style
                    if let Some(cell) = grid.get_cell_mut(*row, *col) {
                        cell.bg_color = new_style.bg_color;
                        cell.fg_color = new_style.fg_color;
                        cell.font_bold = new_style.font_bold;
                        cell.font_italic = new_style.font_italic;
                    }
                }
            }

            // Move action back to undo stack
            self.undo_stack.push(action);
            true
        } else {
            false
        }
    }

    /// Record an action for undo
    pub fn record_action(&mut self, action: EditAction) {
        self.undo_stack.push(action);
        // Clear redo stack when new action is recorded
        self.redo_stack.clear();
    }

    /// Clear undo history
    pub fn clear_undo_history(&mut self) {
        self.undo_stack.clear();
    }

    /// Clear redo history
    pub fn clear_redo_history(&mut self) {
        self.redo_stack.clear();
    }

    /// Get cell style for undo tracking
    pub fn get_cell_style(grid: &Grid, row: usize, col: usize) -> CellStyle {
        if let Some(cell) = grid.get_cell(row, col) {
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
}

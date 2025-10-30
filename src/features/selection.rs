use crate::core::{Cell, Grid};
use std::collections::HashSet;

/// Cell selection functionality for DataGrid
pub struct SelectionState {
    pub selected_cells: HashSet<(usize, usize)>,
    pub selection_anchor: Option<(usize, usize)>,
}

impl Default for SelectionState {
    fn default() -> Self {
        Self {
            selected_cells: HashSet::new(),
            selection_anchor: None,
        }
    }
}

impl SelectionState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Select a single cell (clears previous selection)
    pub fn select_single_cell(&mut self, row: usize, col: usize, grid: &mut Grid) {
        // Clear all previous selections
        self.clear_selection(grid);

        // Add new selection
        self.selected_cells.insert((row, col));
        self.selection_anchor = Some((row, col));

        // Update cell state
        if let Some(cell) = grid.get_cell_mut(row, col) {
            cell.selected = true;
        } else {
            let mut cell = Cell::default();
            cell.selected = true;
            grid.set_cell(row, col, cell);
        }
    }

    /// Toggle cell selection (add/remove from selection)
    pub fn toggle_cell_selection(&mut self, row: usize, col: usize, grid: &mut Grid) {
        if self.selected_cells.contains(&(row, col)) {
            // Remove from selection
            self.selected_cells.remove(&(row, col));
            if let Some(cell) = grid.get_cell_mut(row, col) {
                cell.selected = false;
            }
        } else {
            // Add to selection
            self.selected_cells.insert((row, col));
            if let Some(cell) = grid.get_cell_mut(row, col) {
                cell.selected = true;
            } else {
                let mut cell = Cell::default();
                cell.selected = true;
                grid.set_cell(row, col, cell);
            }
        }

        // Update anchor
        if !self.selected_cells.is_empty() {
            self.selection_anchor = Some((row, col));
        }
    }

    /// Select range from anchor to target cell
    pub fn select_range(&mut self, target_row: usize, target_col: usize, grid: &mut Grid) {
        if let Some((anchor_row, anchor_col)) = self.selection_anchor {
            // Clear previous selection
            self.clear_selection(grid);

            // Calculate range
            let min_row = anchor_row.min(target_row);
            let max_row = anchor_row.max(target_row);
            let min_col = anchor_col.min(target_col);
            let max_col = anchor_col.max(target_col);

            // Select all cells in range
            for r in min_row..=max_row {
                for c in min_col..=max_col {
                    if r < grid.row_count() && c < grid.col_count() {
                        self.selected_cells.insert((r, c));

                        if let Some(cell) = grid.get_cell_mut(r, c) {
                            cell.selected = true;
                        } else {
                            let mut cell = Cell::default();
                            cell.selected = true;
                            grid.set_cell(r, c, cell);
                        }
                    }
                }
            }
        } else {
            // No anchor, just select single cell
            self.select_single_cell(target_row, target_col, grid);
        }
    }

    /// Clear all selections
    pub fn clear_selection(&mut self, grid: &mut Grid) {
        for (row, col) in &self.selected_cells {
            if let Some(cell) = grid.get_cell_mut(*row, *col) {
                cell.selected = false;
            }
        }
        self.selected_cells.clear();
    }

    /// Get selected cells as a JSON array of [row, col] pairs
    pub fn get_selected_cells(&self) -> String {
        let cells: Vec<Vec<usize>> = self.selected_cells
            .iter()
            .map(|(row, col)| vec![*row, *col])
            .collect();
        serde_json::to_string(&cells).unwrap_or_else(|_| "[]".to_string())
    }

    /// Get selection count
    pub fn get_selection_count(&self) -> usize {
        self.selected_cells.len()
    }

    /// Select all cells (Ctrl+A)
    pub fn select_all(&mut self, grid: &mut Grid) {
        self.clear_selection(grid);

        for row in 0..grid.row_count() {
            for col in 0..grid.col_count() {
                self.selected_cells.insert((row, col));
                if let Some(cell) = grid.get_cell_mut(row, col) {
                    cell.selected = true;
                }
            }
        }

        // Set anchor to first cell
        self.selection_anchor = Some((0, 0));
    }

    /// Select entire row
    pub fn select_row(&mut self, row: usize, grid: &mut Grid) {
        if row >= grid.row_count() {
            return;
        }

        self.clear_selection(grid);

        for col in 0..grid.col_count() {
            self.selected_cells.insert((row, col));
            if let Some(cell) = grid.get_cell_mut(row, col) {
                cell.selected = true;
            }
        }

        self.selection_anchor = Some((row, 0));
    }

    /// Select entire column
    pub fn select_col(&mut self, col: usize, grid: &mut Grid) {
        if col >= grid.col_count() {
            return;
        }

        self.clear_selection(grid);

        for row in 0..grid.row_count() {
            self.selected_cells.insert((row, col));
            if let Some(cell) = grid.get_cell_mut(row, col) {
                cell.selected = true;
            }
        }

        self.selection_anchor = Some((0, col));
    }
}

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

    /// Check if a cell is selected
    pub fn is_selected(&self, row: usize, col: usize) -> bool {
        self.selected_cells.contains(&(row, col))
    }

    /// Select a single cell (clears previous selection)
    pub fn select_single_cell(&mut self, row: usize, col: usize) {
        // Clear all previous selections
        self.selected_cells.clear();

        // Add new selection
        self.selected_cells.insert((row, col));
        self.selection_anchor = Some((row, col));
    }

    /// Toggle cell selection (add/remove from selection)
    pub fn toggle_cell_selection(&mut self, row: usize, col: usize) {
        if self.selected_cells.contains(&(row, col)) {
            // Remove from selection
            self.selected_cells.remove(&(row, col));
        } else {
            // Add to selection
            self.selected_cells.insert((row, col));
        }

        // Update anchor
        if !self.selected_cells.is_empty() {
            self.selection_anchor = Some((row, col));
        }
    }

    /// Select range from anchor to target cell
    pub fn select_range(&mut self, target_row: usize, target_col: usize, row_count: usize, col_count: usize) {
        if let Some((anchor_row, anchor_col)) = self.selection_anchor {
            // Clear previous selection
            self.selected_cells.clear();

            // Calculate range
            let min_row = anchor_row.min(target_row);
            let max_row = anchor_row.max(target_row);
            let min_col = anchor_col.min(target_col);
            let max_col = anchor_col.max(target_col);

            // Select all cells in range
            for r in min_row..=max_row {
                for c in min_col..=max_col {
                    if r < row_count && c < col_count {
                        self.selected_cells.insert((r, c));
                    }
                }
            }
        } else {
            // No anchor, just select single cell
            self.select_single_cell(target_row, target_col);
        }
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) {
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
    pub fn select_all(&mut self, row_count: usize, col_count: usize) {
        self.selected_cells.clear();

        for row in 0..row_count {
            for col in 0..col_count {
                self.selected_cells.insert((row, col));
            }
        }

        // Set anchor to first cell
        self.selection_anchor = Some((0, 0));
    }

    /// Select entire row
    pub fn select_row(&mut self, row: usize, row_count: usize, col_count: usize) {
        if row >= row_count {
            return;
        }

        self.selected_cells.clear();

        for col in 0..col_count {
            self.selected_cells.insert((row, col));
        }

        self.selection_anchor = Some((row, 0));
    }

    /// Select entire column
    pub fn select_col(&mut self, col: usize, row_count: usize, col_count: usize) {
        if col >= col_count {
            return;
        }

        self.selected_cells.clear();

        for row in 0..row_count {
            self.selected_cells.insert((row, col));
        }

        self.selection_anchor = Some((0, col));
    }
}

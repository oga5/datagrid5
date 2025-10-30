use crate::core::{cell::CellValue, Grid, Viewport};
use std::collections::HashSet;

/// Search and replace functionality for DataGrid
pub struct SearchState {
    pub search_query: String,
    pub search_results: Vec<(usize, usize)>,
    pub current_search_index: Option<usize>,
    pub search_case_sensitive: bool,
    pub search_whole_word: bool,
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            search_results: Vec::new(),
            current_search_index: None,
            search_case_sensitive: false,
            search_whole_word: false,
        }
    }
}

impl SearchState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Search for text (case-insensitive, substring matching)
    pub fn search_text(&mut self, query: String, grid: &Grid) -> usize {
        self.search_text_with_options(query, false, false, grid)
    }

    /// Search for text with options
    pub fn search_text_with_options(
        &mut self,
        query: String,
        case_sensitive: bool,
        whole_word: bool,
        grid: &Grid,
    ) -> usize {
        self.search_case_sensitive = case_sensitive;
        self.search_whole_word = whole_word;
        self.search_query = if case_sensitive {
            query.clone()
        } else {
            query.to_lowercase()
        };
        self.search_results.clear();
        self.current_search_index = None;

        if query.is_empty() {
            return 0;
        }

        // Search through all cells
        for row in 0..grid.row_count() {
            for col in 0..grid.col_count() {
                let cell_text = grid.get_value_string(row, col);
                let search_text = if case_sensitive {
                    cell_text.clone()
                } else {
                    cell_text.to_lowercase()
                };

                let is_match = if whole_word {
                    // Whole word matching: split by whitespace and check for exact match
                    search_text
                        .split_whitespace()
                        .any(|word| word == self.search_query)
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
        }

        self.search_results.len()
    }

    /// Move to next search result
    pub fn search_next(&mut self) -> Option<(usize, usize)> {
        if self.search_results.is_empty() {
            return None;
        }

        if let Some(current_idx) = self.current_search_index {
            let next_idx = (current_idx + 1) % self.search_results.len();
            self.current_search_index = Some(next_idx);
            Some(self.search_results[next_idx])
        } else {
            None
        }
    }

    /// Move to previous search result
    pub fn search_prev(&mut self) -> Option<(usize, usize)> {
        if self.search_results.is_empty() {
            return None;
        }

        if let Some(current_idx) = self.current_search_index {
            let prev_idx = if current_idx == 0 {
                self.search_results.len() - 1
            } else {
                current_idx - 1
            };
            self.current_search_index = Some(prev_idx);
            Some(self.search_results[prev_idx])
        } else {
            None
        }
    }

    /// Search using regular expression
    pub fn search_regex(
        &mut self,
        pattern: String,
        case_sensitive: bool,
        grid: &Grid,
    ) -> Result<usize, String> {
        use regex::RegexBuilder;

        // Build regex with case sensitivity option
        let regex = match RegexBuilder::new(&pattern)
            .case_insensitive(!case_sensitive)
            .build()
        {
            Ok(re) => re,
            Err(e) => return Err(format!("Invalid regex pattern: {}", e)),
        };

        self.search_query = pattern;
        self.search_case_sensitive = case_sensitive;
        self.search_whole_word = false; // Not applicable for regex
        self.search_results.clear();
        self.current_search_index = None;

        // Search through all cells
        for row in 0..grid.row_count() {
            for col in 0..grid.col_count() {
                let cell_text = grid.get_value_string(row, col);

                if regex.is_match(&cell_text) {
                    self.search_results.push((row, col));
                }
            }
        }

        if !self.search_results.is_empty() {
            self.current_search_index = Some(0);
        }

        Ok(self.search_results.len())
    }

    /// Validate regex pattern without performing search
    pub fn validate_regex_pattern(pattern: &str) -> bool {
        use regex::Regex;
        Regex::new(pattern).is_ok()
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
    pub fn replace_current(&mut self, replacement: String, grid: &mut Grid) -> bool {
        if let Some(idx) = self.current_search_index {
            if idx < self.search_results.len() {
                let (row, col) = self.search_results[idx];

                // Parse replacement as number if possible
                if let Ok(num) = replacement.parse::<f64>() {
                    grid.set_value(row, col, CellValue::Number(num));
                } else {
                    grid.set_value(row, col, CellValue::Text(replacement));
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
    pub fn replace_all(&mut self, replacement: String, grid: &mut Grid) -> usize {
        let count = self.search_results.len();

        // Replace all matching cells
        for (row, col) in &self.search_results {
            // Parse replacement as number if possible
            if let Ok(num) = replacement.parse::<f64>() {
                grid.set_value(*row, *col, CellValue::Number(num));
            } else {
                grid.set_value(*row, *col, CellValue::Text(replacement.clone()));
            }
        }

        // Clear search results after replacing all
        self.clear_search();
        count
    }

    /// Replace in selection only
    pub fn replace_in_selection(
        search: String,
        replacement: String,
        case_sensitive: bool,
        selected_cells: &HashSet<(usize, usize)>,
        grid: &mut Grid,
    ) -> usize {
        let mut count = 0;
        let search_str = if case_sensitive {
            search.clone()
        } else {
            search.to_lowercase()
        };

        // Get list of selected cells
        let selected: Vec<(usize, usize)> = selected_cells.iter().cloned().collect();

        for (row, col) in selected {
            let cell_text = grid.get_value_string(row, col);
            let search_text = if case_sensitive {
                cell_text.clone()
            } else {
                cell_text.to_lowercase()
            };

            if search_text.contains(&search_str) {
                // Parse replacement as number if possible
                if let Ok(num) = replacement.parse::<f64>() {
                    grid.set_value(row, col, CellValue::Number(num));
                } else {
                    grid.set_value(row, col, CellValue::Text(replacement.clone()));
                }
                count += 1;
            }
        }

        count
    }
}

/// Helper to ensure a cell is visible in the viewport
pub fn ensure_cell_visible(row: usize, col: usize, grid: &Grid, viewport: &mut Viewport) {
    let cell_x = grid.col_x_position(col);
    let cell_y = grid.row_y_position(row);
    let cell_width = grid.col_width(col);
    let cell_height = grid.row_height(row);

    let mut scroll_x = viewport.scroll_x;
    let mut scroll_y = viewport.scroll_y;

    // Check horizontal visibility
    if cell_x < scroll_x {
        scroll_x = cell_x;
    } else if cell_x + cell_width > scroll_x + viewport.canvas_width {
        scroll_x = cell_x + cell_width - viewport.canvas_width;
    }

    // Check vertical visibility
    if cell_y < scroll_y {
        scroll_y = cell_y;
    } else if cell_y + cell_height > scroll_y + viewport.canvas_height {
        scroll_y = cell_y + cell_height - viewport.canvas_height;
    }

    // Update scroll if changed
    if scroll_x != viewport.scroll_x || scroll_y != viewport.scroll_y {
        viewport.set_scroll(scroll_x, scroll_y, grid);
        viewport.update_visible_range(grid);
    }
}

use crate::core::{cell::CellValue, Grid};
use std::collections::HashSet;

/// Clipboard operations for copying and pasting cells
pub struct ClipboardOps;

impl ClipboardOps {
    /// Copy selected cells to TSV (Tab-Separated Values) format
    pub fn copy_selected_cells(
        selected_cells: &HashSet<(usize, usize)>,
        grid: &Grid,
    ) -> String {
        if selected_cells.is_empty() {
            return String::new();
        }

        // Sort selected cells by row, then by column
        let mut cells: Vec<(usize, usize)> = selected_cells.iter().copied().collect();
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
                if selected_cells.contains(&(row, col)) {
                    let value = grid.get_value_string(row, col);
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
    pub fn cut_selected_cells(
        selected_cells: &HashSet<(usize, usize)>,
        grid: &mut Grid,
    ) -> String {
        // First copy the cells
        let clipboard_text = Self::copy_selected_cells(selected_cells, grid);

        // Then clear all selected cells
        let cells_to_clear: Vec<(usize, usize)> = selected_cells.iter().copied().collect();
        for (row, col) in cells_to_clear {
            grid.set_value(row, col, CellValue::Empty);
        }

        clipboard_text
    }

    /// Paste cells from TSV (Tab-Separated Values) format
    pub fn paste_cells(
        tsv_text: String,
        selection_anchor: Option<(usize, usize)>,
        selected_cells: &HashSet<(usize, usize)>,
        grid: &mut Grid,
    ) -> Result<(), String> {
        if tsv_text.is_empty() {
            return Ok(());
        }

        // Determine starting position (focus cell or first selected cell)
        let (start_row, start_col) = if let Some(anchor) = selection_anchor {
            anchor
        } else if !selected_cells.is_empty() {
            let mut cells: Vec<(usize, usize)> = selected_cells.iter().copied().collect();
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
            if target_row >= grid.row_count() {
                break; // Don't paste beyond grid bounds
            }

            let values: Vec<&str> = line.split('\t').collect();
            for (col_offset, value) in values.iter().enumerate() {
                let target_col = start_col + col_offset;
                if target_col >= grid.col_count() {
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

                grid.set_value(target_row, target_col, cell_value);
            }
        }

        Ok(())
    }
}

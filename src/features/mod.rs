// Feature modules for DataGrid functionality
// This module contains specialized functionality extracted from lib.rs

pub mod clipboard;
pub mod editing;
pub mod resize;
pub mod search;
pub mod selection;
pub mod undo_redo;

// Re-export commonly used types
pub use clipboard::ClipboardOps;
pub use editing::EditingState;
pub use resize::ResizeState;
pub use search::{ensure_cell_visible, SearchState};
pub use selection::SelectionState;
pub use undo_redo::{CellStyle, EditAction, UndoRedoState};

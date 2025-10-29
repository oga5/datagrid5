# lib.rs Refactoring Plan

## Problem
lib.rs has grown to 3150 lines with 145 public functions, making it difficult to maintain and navigate.

## Solution
Extract functionality into focused feature modules under `src/features/`.

## Completed Modules

### 1. features/editing.rs (~110 lines)
**Purpose**: Cell editing operations

**Extracted Functions**:
- `EditingState` struct to manage editing state
- `start_edit()` - Start editing a cell
- `end_edit()` - End editing mode
- `is_editing()` - Check if currently editing
- `update_cell_value()` - Update cell value during editing
- `get_cell_edit_rect()` - Get cell position for editor
- `handle_double_click()` - Handle double-click for editing

**Benefits**:
- Isolates all editing logic in one place
- Makes editing state management explicit
- Easier to test editing functionality independently

### 2. features/selection.rs (~200 lines)
**Purpose**: Cell/row/column selection management

**Extracted Functions**:
- `SelectionState` struct to manage selection state
- `select_single_cell()` - Select a single cell
- `toggle_cell_selection()` - Toggle cell selection
- `select_range()` - Select range from anchor to target
- `clear_selection()` - Clear all selections
- `get_selected_cells()` - Get selected cells as JSON
- `get_selection_count()` - Get selection count
- `select_all()` - Select all cells (Ctrl+A)
- `select_row()` - Select entire row
- `select_col()` - Select entire column

**Benefits**:
- Centralizes all selection logic
- Makes selection state explicit and manageable
- Easier to add new selection features

### 3. features/clipboard.rs (~140 lines)
**Purpose**: Copy/cut/paste operations

**Extracted Functions**:
- `ClipboardOps` utility struct
- `copy_selected_cells()` - Copy cells to TSV format
- `cut_selected_cells()` - Cut cells (copy and clear)
- `paste_cells()` - Paste cells from TSV format

**Benefits**:
- Isolates clipboard operations
- TSV parsing logic is now separate
- Easier to add new clipboard formats

## Remaining Work

### 4. features/undo_redo.rs (TODO ~200-300 lines)
**Functions to extract**:
- `EditAction` enum
- `undo()`
- `redo()`
- Undo stack management

### 5. features/search.rs (TODO ~300-400 lines)
**Functions to extract**:
- `search_text()`
- `search_text_with_options()`
- `search_next()`
- `search_prev()`
- `search_regex()`
- `replace_current()`
- `replace_all()`
- `replace_in_selection()`

### 6. features/resize.rs (TODO ~200-300 lines)
**Functions to extract**:
- `check_resize_handle()`
- `start_resize()`
- `update_resize()`
- `end_resize()`
- `is_resizing()`

### 7. Refactor lib.rs (TODO)
**Changes needed**:
1. Update `DataGrid` struct to use new state structs:
   ```rust
   pub struct DataGrid {
       // ... existing fields ...
       editing: EditingState,
       selection: SelectionState,
       // ... other state ...
   }
   ```

2. Update `impl DataGrid` methods to delegate to feature modules:
   ```rust
   pub fn start_edit(&mut self, row: usize, col: usize) -> bool {
       self.editing.start_edit(row, col, &self.grid)
   }
   ```

3. Keep core functionality in lib.rs:
   - DataGrid struct definition
   - new() constructors
   - render()
   - resize()
   - Core event handling coordination

## Benefits of Refactoring

1. **Improved Maintainability**
   - Each module has a single, clear responsibility
   - Easier to find and modify specific functionality
   - Reduced cognitive load when reading code

2. **Better Testability**
   - Feature modules can be tested independently
   - Easier to mock dependencies
   - More focused unit tests

3. **Enhanced Collaboration**
   - Multiple developers can work on different modules
   - Reduced merge conflicts
   - Clearer code ownership

4. **Easier Onboarding**
   - New developers can understand one module at a time
   - Module boundaries make the architecture clear
   - Better documentation opportunities

5. **Future-Proof Architecture**
   - Easy to add new features as new modules
   - Can refactor individual modules without affecting others
   - Supports gradual migration to async/await if needed

## File Size Comparison

**Before**:
- lib.rs: 3150 lines

**After** (projected):
- lib.rs: ~800-1000 lines (core only)
- features/editing.rs: ~110 lines
- features/selection.rs: ~200 lines
- features/clipboard.rs: ~140 lines
- features/undo_redo.rs: ~250 lines (TODO)
- features/search.rs: ~350 lines (TODO)
- features/resize.rs: ~250 lines (TODO)
- **Total**: ~2100 lines across 7 files

**Reduction**: From one 3150-line file to manageable modules averaging ~300 lines each.

## Migration Strategy

1. ✅ Create feature modules with extracted functionality
2. ✅ Create mod.rs with re-exports
3. ⏳ Update lib.rs to use feature modules (delegate to them)
4. ⏳ Test that everything still compiles
5. ⏳ Run existing tests to ensure functionality is preserved
6. ⏳ Update documentation to reflect new structure

## Notes

- All public APIs remain unchanged - this is an internal refactoring
- WASM bindings continue to work as before
- No breaking changes for JavaScript consumers
- Can be done incrementally - one module at a time

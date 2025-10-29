# Refactoring Next Steps

## Current Status ✅

All feature modules have been successfully extracted:

1. **features/editing.rs** (~110 lines) - Cell editing operations
2. **features/selection.rs** (~200 lines) - Selection management
3. **features/clipboard.rs** (~140 lines) - Copy/cut/paste operations
4. **features/undo_redo.rs** (~230 lines) - Undo/redo functionality
5. **features/search.rs** (~370 lines) - Search and replace operations
6. **features/resize.rs** (~155 lines) - Column/row resizing

**DataGrid struct** has been updated to use the new state structs:
```rust
pub struct DataGrid {
    // ... core fields ...
    editing: EditingState,
    selection: SelectionState,
    resize: ResizeState,
    search: SearchState,
    undo_redo: UndoRedoState,
    // ... other fields ...
}
```

## Remaining Work

### 1. Update Constructors (CRITICAL)

The DataGrid constructors need to initialize the new state structs:

**Files to update:**
- `DataGrid::from_container()` (line ~68)
- `DataGrid::new()` (line ~230)

**Required changes:**
```rust
// Replace old individual field initialization:
is_editing: false,
editing_cell: None,
is_resizing: false,
resizing_column: None,
// ... etc

// With new state struct initialization:
editing: EditingState::new(),
selection: SelectionState::new(),
resize: ResizeState::new(),
search: SearchState::new(),
undo_redo: UndoRedoState::new(),
```

### 2. Update Method Delegation (145 methods)

All public methods that used the old fields must now delegate to the feature modules.

#### Editing Methods (~6 methods)
```rust
// OLD:
pub fn start_edit(&mut self, row: usize, col: usize) -> bool {
    if row >= self.grid.row_count() || col >= self.grid.col_count() {
        return false;
    }
    // ... more logic ...
    self.is_editing = true;
    self.editing_cell = Some((row, col));
    true
}

// NEW:
pub fn start_edit(&mut self, row: usize, col: usize) -> bool {
    self.editing.start_edit(row, col, &self.grid)
}

pub fn end_edit(&mut self) {
    self.editing.end_edit()
}

pub fn is_editing(&self) -> bool {
    self.editing.is_editing()
}

pub fn update_cell_value(&mut self, row: usize, col: usize, value: String) {
    self.editing.update_cell_value(row, col, value, &mut self.grid)
}

pub fn get_cell_edit_rect(&self, row: usize, col: usize) -> Vec<f32> {
    self.editing.get_cell_edit_rect(row, col, &self.grid, &self.viewport)
}

pub fn handle_double_click(&mut self, event: MouseEvent) -> Option<Vec<usize>> {
    self.editing.handle_double_click(event, &self.grid, &self.viewport)
}
```

#### Selection Methods (~10 methods)
```rust
// OLD: self.selected_cells, self.selection_anchor
// NEW: self.selection.selected_cells, self.selection.selection_anchor

pub fn select_all(&mut self) {
    self.selection.select_all(&mut self.grid)
}

pub fn select_row(&mut self, row: usize) {
    self.selection.select_row(row, &mut self.grid)
}

pub fn select_col(&mut self, col: usize) {
    self.selection.select_col(col, &mut self.grid)
}

pub fn get_selected_cells(&self) -> String {
    self.selection.get_selected_cells()
}

pub fn get_selection_count(&self) -> usize {
    self.selection.get_selection_count()
}

// Private methods:
fn select_single_cell(&mut self, row: usize, col: usize) {
    self.selection.select_single_cell(row, col, &mut self.grid)
}

fn toggle_cell_selection(&mut self, row: usize, col: usize) {
    self.selection.toggle_cell_selection(row, col, &mut self.grid)
}

fn select_range(&mut self, target_row: usize, target_col: usize) {
    self.selection.select_range(target_row, target_col, &mut self.grid)
}

fn clear_selection(&mut self) {
    self.selection.clear_selection(&mut self.grid)
}
```

#### Clipboard Methods (~3 methods)
```rust
pub fn copy_selected_cells(&self) -> String {
    ClipboardOps::copy_selected_cells(&self.selection.selected_cells, &self.grid)
}

pub fn cut_selected_cells(&mut self) -> String {
    ClipboardOps::cut_selected_cells(&self.selection.selected_cells, &mut self.grid)
}

pub fn paste_cells(&mut self, tsv_text: String) -> Result<(), String> {
    ClipboardOps::paste_cells(
        tsv_text,
        self.selection.selection_anchor,
        &self.selection.selected_cells,
        &mut self.grid,
    )
}
```

#### Resize Methods (~5 methods)
```rust
// OLD: self.is_resizing, self.resizing_column, self.resizing_row, etc.
// NEW: self.resize.*

pub fn check_resize_handle(&self, x: f32, y: f32) -> String {
    self.resize.check_resize_handle(x, y, &self.grid, &self.viewport)
}

pub fn start_resize(&mut self, x: f32, y: f32, resize_type: &str) -> bool {
    self.resize.start_resize(x, y, resize_type, &self.grid)
}

pub fn update_resize(&mut self, x: f32, y: f32) {
    self.resize.update_resize(x, y, &mut self.grid)
}

pub fn end_resize(&mut self) {
    self.resize.end_resize()
}

pub fn is_resizing(&self) -> bool {
    self.resize.is_resizing()
}
```

#### Search Methods (~15 methods)
```rust
// OLD: self.search_query, self.search_results, etc.
// NEW: self.search.*

pub fn search_text(&mut self, query: String) -> usize {
    let count = self.search.search_text(query, &self.grid);
    if count > 0 && self.search.current_search_index.is_some() {
        let (row, col) = self.search.search_results[0];
        self.selection.select_single_cell(row, col, &mut self.grid);
        features::search::ensure_cell_visible(row, col, &self.grid, &mut self.viewport);
    }
    count
}

pub fn search_text_with_options(&mut self, query: String, case_sensitive: bool, whole_word: bool) -> usize {
    let count = self.search.search_text_with_options(query, case_sensitive, whole_word, &self.grid);
    if count > 0 && self.search.current_search_index.is_some() {
        let (row, col) = self.search.search_results[0];
        self.selection.select_single_cell(row, col, &mut self.grid);
        features::search::ensure_cell_visible(row, col, &self.grid, &mut self.viewport);
    }
    count
}

pub fn search_next(&mut self) -> bool {
    if let Some((row, col)) = self.search.search_next() {
        self.selection.select_single_cell(row, col, &mut self.grid);
        features::search::ensure_cell_visible(row, col, &self.grid, &mut self.viewport);
        true
    } else {
        false
    }
}

pub fn search_prev(&mut self) -> bool {
    if let Some((row, col)) = self.search.search_prev() {
        self.selection.select_single_cell(row, col, &mut self.grid);
        features::search::ensure_cell_visible(row, col, &self.grid, &mut self.viewport);
        true
    } else {
        false
    }
}

pub fn search_regex(&mut self, pattern: String, case_sensitive: bool) -> Result<usize, String> {
    let result = self.search.search_regex(pattern, case_sensitive, &self.grid)?;
    if result > 0 && self.search.current_search_index.is_some() {
        let (row, col) = self.search.search_results[0];
        self.selection.select_single_cell(row, col, &mut self.grid);
        features::search::ensure_cell_visible(row, col, &self.grid, &mut self.viewport);
    }
    Ok(result)
}

pub fn validate_regex_pattern(&self, pattern: String) -> bool {
    SearchState::validate_regex_pattern(&pattern)
}

pub fn clear_search(&mut self) {
    self.search.clear_search()
}

pub fn get_search_result_count(&self) -> usize {
    self.search.get_search_result_count()
}

pub fn get_current_search_index(&self) -> i32 {
    self.search.get_current_search_index()
}

pub fn is_search_result(&self, row: usize, col: usize) -> bool {
    self.search.is_search_result(row, col)
}

pub fn is_current_search_result(&self, row: usize, col: usize) -> bool {
    self.search.is_current_search_result(row, col)
}

pub fn replace_current(&mut self, replacement: String) -> bool {
    self.search.replace_current(replacement, &mut self.grid)
}

pub fn replace_all(&mut self, replacement: String) -> usize {
    self.search.replace_all(replacement, &mut self.grid)
}

pub fn replace_in_selection(&mut self, search: String, replacement: String, case_sensitive: bool) -> usize {
    SearchState::replace_in_selection(
        search,
        replacement,
        case_sensitive,
        &self.selection.selected_cells,
        &mut self.grid,
    )
}
```

#### Undo/Redo Methods (~2 methods + helpers)
```rust
// OLD: self.undo_stack, self.redo_stack
// NEW: self.undo_redo.*

pub fn undo(&mut self) -> bool {
    self.undo_redo.undo(&mut self.grid, &mut self.viewport)
}

pub fn redo(&mut self) -> bool {
    self.undo_redo.redo(&mut self.grid, &mut self.viewport)
}

// Helper method:
fn get_cell_style(&self, row: usize, col: usize) -> CellStyle {
    UndoRedoState::get_cell_style(&self.grid, row, col)
}
```

#### Other Methods Needing Updates

Methods that reference the old fields need to be updated:

**Search-related in render():**
```rust
self.text_renderer.render_with_search(
    &self.grid,
    &self.viewport,
    &self.search.search_results,  // Changed from self.search_results
    self.search.current_search_index  // Changed from self.current_search_index
);
```

**Selection-related methods:**
- `handle_mouse_down_with_modifiers()`
- `handle_mouse_down_at()`
- Any method accessing `self.selected_cells` or `self.selection_anchor`

**Editing-related methods:**
- Any method checking `self.is_editing`
- Methods in keyboard/mouse handlers

### 3. Search and Replace All Old References

Use global search-replace to update all remaining references:

```bash
# In lib.rs:
self.is_editing → self.editing.is_editing
self.editing_cell → self.editing.editing_cell
self.is_resizing → self.resize.is_resizing
self.resizing_column → self.resize.resizing_column
self.resizing_row → self.resize.resizing_row
self.resize_start_pos → self.resize.resize_start_pos
self.resize_start_size → self.resize.resize_start_size
self.selected_cells → self.selection.selected_cells
self.selection_anchor → self.selection.selection_anchor
self.search_query → self.search.search_query
self.search_results → self.search.search_results
self.current_search_index → self.search.current_search_index
self.search_case_sensitive → self.search.search_case_sensitive
self.search_whole_word → self.search.search_whole_word
self.undo_stack → self.undo_redo.undo_stack
self.redo_stack → self.undo_redo.redo_stack
```

### 4. Test Compilation

After making all changes:

```bash
cargo check
cargo build --target wasm32-unknown-unknown
# If wasm-pack is available:
./build.sh
```

Fix any compilation errors that arise.

### 5. Verify Functionality

Run examples to ensure:
- Cell editing still works
- Selection works correctly
- Copy/paste works
- Search/replace works
- Undo/redo works
- Resize works

## Implementation Strategy

1. **Phase 1: Constructors** (CRITICAL - do this first)
   - Update `from_container()`
   - Update `new()`
   - Test compilation

2. **Phase 2: Simple Methods** (easiest)
   - Editing methods (6 methods)
   - Resize methods (5 methods)
   - Get/set/is methods (simple getters)

3. **Phase 3: Complex Methods**
   - Selection methods (10+ methods)
   - Search methods (15+ methods)
   - Clipboard methods (3 methods)

4. **Phase 4: Integration Points**
   - render() method
   - Event handlers
   - Methods that coordinate multiple features

5. **Phase 5: Testing**
   - Compile and fix errors
   - Test each feature
   - Run examples

## Benefits After Completion

- **Reduced file size**: lib.rs from 3150 lines → ~1500 lines (52% reduction)
- **Improved maintainability**: Each feature in a focused module
- **Better testability**: Features can be unit tested independently
- **Clearer architecture**: State management is explicit
- **Easier collaboration**: Multiple developers can work on different features
- **Future-proof**: Easy to add new features or refactor existing ones

## Notes

- This is a **non-breaking change** - all public APIs remain the same
- JavaScript/WASM bindings are unchanged
- Only internal structure is being reorganized
- Can be done incrementally and tested at each step

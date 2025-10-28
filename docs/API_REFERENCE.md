# DataGrid5 API Reference

Complete API documentation for DataGrid5 WebAssembly grid control.

[日本語版](./API_REFERENCE.ja.md)

## Table of Contents

- [Initialization](#initialization)
- [Grid Configuration](#grid-configuration)
- [Data Management](#data-management)
- [Rendering](#rendering)
- [Event Handling](#event-handling)
- [Editing](#editing)
- [Selection](#selection)
- [Search & Replace](#search--replace)
- [Sorting & Filtering](#sorting--filtering)
- [Styling](#styling)
- [Undo/Redo](#undoredo)
- [Performance](#performance)
- [Worker Thread Support](#worker-thread-support)
- [Context Menu](#context-menu)

---

## Initialization

### `DataGrid.from_container(container_id, options_json)`

Creates a new DataGrid instance from a container div.

**Parameters:**
- `container_id: string` - ID of the container div element
- `options_json: string` - JSON string containing grid configuration

**Returns:** `DataGrid` instance

**Example:**
```javascript
const options = {
    rows: 100,
    cols: 10,
    width: 800,
    height: 600,
    columns: [
        {
            display_name: "ID",
            internal_name: "id",
            width: 60,
            data_type: "number",
            editable: false
        }
    ],
    frozen_rows: 1,
    frozen_cols: 0,
    readonly: false
};

const grid = DataGrid.from_container('my-grid', JSON.stringify(options));
```

### `new DataGrid(webgl_canvas_id, text_canvas_id, rows, cols)`

Creates a new DataGrid instance with explicit canvas IDs (legacy method).

**Parameters:**
- `webgl_canvas_id: string` - ID of the WebGL canvas element
- `text_canvas_id: string` - ID of the text overlay canvas element
- `rows: number` - Number of rows
- `cols: number` - Number of columns

**Returns:** `DataGrid` instance

---

## Grid Configuration

### Column Configuration Options

```typescript
interface ColumnConfig {
    display_name: string;      // Display name in header
    internal_name: string;     // Unique internal identifier
    width: number;             // Column width in pixels
    data_type: "text" | "number" | "date" | "boolean";
    editable: boolean;         // Can cells be edited
    visible: boolean;          // Is column visible
    sortable: boolean;         // Can column be sorted
    filterable: boolean;       // Can column be filtered
}
```

### Grid Options

```typescript
interface GridOptions {
    rows: number;              // Number of rows
    cols: number;              // Number of columns
    width: number;             // Grid width in pixels
    height: number;            // Grid height in pixels
    columns?: ColumnConfig[];  // Column configurations

    // Frozen panes
    frozen_rows?: number;      // Number of frozen rows (default: 0)
    frozen_cols?: number;      // Number of frozen columns (default: 0)

    // Display options
    show_headers?: boolean;    // Show row/column headers (default: true)
    show_grid_lines?: boolean; // Show grid lines (default: true)
    alternate_row_colors?: boolean; // Alternate row colors (default: false)

    // Interaction
    readonly?: boolean;        // Read-only mode (default: false)
    enable_context_menu?: boolean; // Enable context menus (default: true)
    enable_row_selection?: boolean; // Enable row selection (default: true)
    enable_col_selection?: boolean; // Enable column selection (default: true)

    // Header dimensions
    row_header_width?: number; // Row header width (default: 60)
    col_header_height?: number; // Column header height (default: 30)
}
```

---

## Data Management

### `load_data_json(data_json)`

Load grid data from JSON.

**Parameters:**
- `data_json: string` - JSON array of cell data

**Format:**
```javascript
const data = [
    { row: 0, col: 0, value: "Text" },
    { row: 0, col: 1, value: 123.45 },
    { row: 0, col: 2, value: "2024-01-15" }, // Date
    { row: 0, col: 3, value: true }          // Boolean
];

grid.load_data_json(JSON.stringify(data));
```

### `set_cell_value(row, col, value)`

Set value of a single cell.

**Parameters:**
- `row: number` - Row index (0-based)
- `col: number` - Column index (0-based)
- `value: string` - Cell value (auto-converted based on column type)

### `get_cell_value(row, col)`

Get value of a cell.

**Parameters:**
- `row: number` - Row index
- `col: number` - Column index

**Returns:** `string` - Cell value

### `get_dimensions()`

Get grid dimensions.

**Returns:** `[number, number]` - [rows, cols]

### `clear_all()`

Clear all cell data.

---

## Rendering

### `render()`

Render the grid. Call this after any data or configuration changes.

**Example:**
```javascript
grid.render(); // Single render

// Or use animation frame for continuous rendering
function renderLoop() {
    grid.render();
    requestAnimationFrame(renderLoop);
}
renderLoop();
```

### `resize(width, height)`

Resize the grid.

**Parameters:**
- `width: number` - New width in pixels
- `col: number` - New height in pixels

---

## Event Handling

### `handle_wheel(event)`

Handle mouse wheel event for scrolling.

**Parameters:**
- `event: WheelEvent` - Mouse wheel event

**Example:**
```javascript
canvas.addEventListener('wheel', (e) => {
    e.preventDefault();
    grid.handle_wheel(e);
});
```

### `handle_mouse_down(event)`

Handle mouse down event.

**Parameters:**
- `event: MouseEvent` - Mouse event

### `handle_mouse_up(event)`

Handle mouse up event.

### `handle_mouse_move(event)`

Handle mouse move event.

### `handle_keyboard(event)`

Handle keyboard event.

**Parameters:**
- `event: KeyboardEvent` - Keyboard event

**Supported Keys:**
- Arrow keys: Navigation
- Ctrl+C: Copy
- Ctrl+V: Paste
- Ctrl+X: Cut
- Ctrl+Z: Undo
- Ctrl+Y: Redo
- Delete: Clear cell
- Enter: Start editing
- Escape: Cancel editing

### `handle_context_menu(event)`

Handle context menu (right-click) event.

**Parameters:**
- `event: MouseEvent` - Mouse event

**Returns:** `string` - JSON containing context information

---

## Editing

### `start_editing(row, col)`

Start editing a cell.

**Parameters:**
- `row: number` - Row index
- `col: number` - Column index

### `stop_editing()`

Stop editing and save changes.

### `cancel_editing()`

Cancel editing without saving.

---

## Selection

### `select_cell(row, col)`

Select a single cell.

**Parameters:**
- `row: number` - Row index
- `col: number` - Column index

### `select_range(start_row, start_col, end_row, end_col)`

Select a range of cells.

### `select_all()`

Select all cells.

### `select_row(row)`

Select an entire row.

### `select_column(col)`

Select an entire column.

### `get_selected_cells()`

Get array of selected cell coordinates.

**Returns:** `Array<[number, number]>` - Array of [row, col] pairs

### `clear_selection()`

Clear current selection.

---

## Search & Replace

### `search(query, case_sensitive, whole_word, use_regex)`

Search for text in grid.

**Parameters:**
- `query: string` - Search query
- `case_sensitive: boolean` - Case-sensitive search
- `whole_word: boolean` - Match whole words only
- `use_regex: boolean` - Use regular expression

**Returns:** `number` - Number of matches found

### `find_next()`

Navigate to next search result.

**Returns:** `boolean` - True if match found

### `find_previous()`

Navigate to previous search result.

**Returns:** `boolean` - True if match found

### `replace(replacement)`

Replace current match.

**Parameters:**
- `replacement: string` - Replacement text

### `replace_all(query, replacement, case_sensitive)`

Replace all matches.

**Parameters:**
- `query: string` - Search query
- `replacement: string` - Replacement text
- `case_sensitive: boolean` - Case-sensitive search

**Returns:** `number` - Number of replacements made

---

## Sorting & Filtering

### `sort_by_column(col, ascending)`

Sort by single column.

**Parameters:**
- `col: number` - Column index
- `ascending: boolean` - Sort direction

### `sort_by_columns(columns)`

Multi-column sort.

**Parameters:**
- `columns: Array<[number, boolean]>` - Array of [col, ascending] pairs

### `filter_by_column(col, predicate)`

Filter rows by column value.

**Parameters:**
- `col: number` - Column index
- `predicate: string` - Filter predicate (text match)

### `clear_filters()`

Clear all filters.

---

## Styling

### `set_cell_bg_color(row, col, color)`

Set cell background color.

**Parameters:**
- `row: number` - Row index
- `col: number` - Column index
- `color: number` - RGBA color as u32 (0xRRGGBBAA)

### `set_cell_fg_color(row, col, color)`

Set cell foreground (text) color.

### `set_cell_font_style(row, col, bold, italic)`

Set cell font style.

**Parameters:**
- `row: number` - Row index
- `col: number` - Column index
- `bold: boolean` - Bold text
- `italic: boolean` - Italic text

### `set_cell_border(row, col, side, color, width)`

Set cell border.

**Parameters:**
- `row: number` - Row index
- `col: number` - Column index
- `side: string` - "top" | "right" | "bottom" | "left"
- `color: number` - RGBA color
- `width: number` - Border width in pixels

---

## Undo/Redo

### `undo()`

Undo last action.

**Returns:** `boolean` - True if undo was performed

### `redo()`

Redo last undone action.

**Returns:** `boolean` - True if redo was performed

### `can_undo()`

Check if undo is available.

**Returns:** `boolean`

### `can_redo()`

Check if redo is available.

**Returns:** `boolean`

---

## Performance

### `get_current_fps()`

Get current rendering FPS.

**Returns:** `number` - FPS value

### `get_render_time()`

Get last frame render time.

**Returns:** `number` - Render time in milliseconds

### `get_memory_usage()`

Get approximate memory usage.

**Returns:** `number` - Memory usage in bytes

### `reserve_capacity(expected_cells)`

Reserve memory capacity.

**Parameters:**
- `expected_cells: number` - Expected number of cells

### `compact_memory()`

Compact memory usage.

---

## Worker Thread Support

### `export_grid_data_json()`

Export all grid data as JSON for worker processing.

**Returns:** `string` - JSON array of cell data

### `export_range_json(start_row, end_row, start_col, end_col)`

Export specific range as JSON.

**Returns:** `string` - JSON array of cell data

### `import_worker_result(result_json)`

Import processed data from worker.

**Parameters:**
- `result_json: string` - JSON array from worker

**Returns:** `number` - Number of cells updated

### `apply_sorted_indices(indices_json)`

Apply sorted row order from worker.

**Parameters:**
- `indices_json: string` - JSON array of row indices

---

## Context Menu

### `get_row_context_operations(row)`

Get available operations for row context menu.

**Parameters:**
- `row: number` - Row index

**Returns:** `Array<string>` - Array of operation names

### `execute_row_operation(operation, row)`

Execute row context menu operation.

**Parameters:**
- `operation: string` - Operation name
- `row: number` - Row index

**Operations:**
- `"insert_row_above"` - Insert row above
- `"insert_row_below"` - Insert row below
- `"delete_row"` - Delete row
- `"copy_row"` - Copy row to clipboard
- `"cut_row"` - Cut row to clipboard
- `"move_row_up"` - Move row up
- `"move_row_down"` - Move row down

---

## Additional Methods

### Row/Column Operations

- `insert_row(index)` - Insert new row
- `delete_row(index)` - Delete row
- `insert_column(index)` - Insert new column
- `delete_column(index)` - Delete column

### Freezing

- `freeze_rows(count)` - Freeze top rows
- `freeze_columns(count)` - Freeze left columns

### Viewport

- `scroll_to(row, col)` - Scroll to cell
- `get_visible_range()` - Get visible cell range

---

For more examples, see the [Examples Guide](./EXAMPLES.md).

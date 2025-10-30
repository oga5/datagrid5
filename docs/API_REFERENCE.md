# DataGrid5 API Reference

Complete API documentation for DataGrid5 WebAssembly grid control.

[日本語版](./API_REFERENCE.ja.md)

## Table of Contents

- [Quick Start with DataGridWrapper](#quick-start-with-datagridwrapper)
- [DataGridWrapper API](#datagridwrapper-api)
- [Low-Level DataGrid API](#low-level-datagrid-api)
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

## Quick Start with DataGridWrapper

**DataGridWrapper** is the recommended high-level API that simplifies DataGrid5 usage by 50-80% less code:

```javascript
import init, { DataGrid } from './pkg/datagrid5.js';

// Initialize WASM
await init();

// Create wrapper with minimal configuration
const wrapper = new DataGridWrapper('grid-container', DataGrid, {
    rows: 100,
    cols: 10,
    enableEditing: true  // Optional: enable cell editing
});

// Load data
const data = [
    { row: 0, col: 0, value: "Hello" },
    { row: 0, col: 1, value: "World" }
];
wrapper.loadData(data);

// That's it! Wrapper handles:
// - Canvas setup and DOM structure
// - Event handlers (mouse, keyboard, wheel)
// - Virtual scrolling
// - Resize handling
// - Clipboard operations (Ctrl+C/V/X)
// - Rendering on demand
```

**Key Benefits:**
- ✅ Automatic canvas and DOM setup
- ✅ Built-in event handling
- ✅ Virtual scrolling out of the box
- ✅ Excel-like keyboard shortcuts
- ✅ Responsive resize support
- ✅ No manual render loop needed

---

## DataGridWrapper API

### Constructor

```javascript
new DataGridWrapper(containerId, DataGrid, options)
```

**Parameters:**
- `containerId: string` - ID of container div
- `DataGrid: class` - DataGrid class from WASM
- `options: object` - Configuration options

**Options:**
```typescript
interface WrapperOptions {
    rows: number;              // Number of rows
    cols: number;              // Number of columns
    width?: number;            // Initial width (default: container width)
    height?: number;           // Initial height (default: container height)
    enableEditing?: boolean;   // Enable cell editing (default: false)
    columns?: ColumnConfig[];  // Column configurations
    frozen_rows?: number;      // Frozen rows (default: 0)
    frozen_cols?: number;      // Frozen columns (default: 0)
}
```

### Methods

#### `loadData(data)`
Load grid data.
```javascript
wrapper.loadData([
    { row: 0, col: 0, value: "Text" },
    { row: 0, col: 1, value: 123 }
]);
```

#### `setCellValue(row, col, value)`
Set individual cell value.
```javascript
wrapper.setCellValue(0, 0, "New Value");
```

#### `getCellValue(row, col)`
Get cell value.
```javascript
const value = wrapper.getCellValue(0, 0);
```

#### `resize(width, height)`
Resize the grid.
```javascript
wrapper.resize(1000, 600);
```

#### `setZebraColor(color)`
Set zebra striping color for alternate rows.
```javascript
wrapper.setZebraColor(0xF5F5F5FF); // Light gray
```

#### `destroy()`
Clean up resources.
```javascript
wrapper.destroy();
```

### Built-in Features

**Keyboard Shortcuts:**
- `Ctrl+C` - Copy selected cells
- `Ctrl+X` - Cut selected cells
- `Ctrl+V` - Paste from clipboard
- Arrow keys - Navigate cells
- `Shift+Arrow` - Extend selection
- `Enter` - Start editing (if enabled)
- `Escape` - Cancel editing

**Mouse Operations:**
- Click - Select cell
- Drag - Select range
- Shift+Click - Extend selection
- Ctrl+Click - Multi-select
- Double-click - Start editing (if enabled)
- Wheel - Scroll grid
- Drag column/row borders - Resize

**Clipboard:**
- TSV (Tab-Separated Values) format
- Compatible with Excel/Google Sheets
- Range copy/paste support

---

## Low-Level DataGrid API

For advanced use cases, you can use the low-level DataGrid API directly. **Note:** Most users should use DataGridWrapper instead.

### Initialization

#### `DataGrid.from_container(container_id, options_json)`

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

#### `new DataGrid(webgl_canvas_id, text_canvas_id, rows, cols)`

Creates a new DataGrid instance with explicit canvas IDs (legacy method).

**Parameters:**
- `webgl_canvas_id: string` - ID of the WebGL canvas element
- `text_canvas_id: string` - ID of the text overlay canvas element
- `rows: number` - Number of rows
- `cols: number` - Number of columns

**Returns:** `DataGrid` instance

---

### Grid Configuration

#### Column Configuration Options

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

#### Grid Options

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

### Data Management

#### `load_data_json(data_json)`

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

#### `set_cell_value(row, col, value)`

Set value of a single cell.

**Parameters:**
- `row: number` - Row index (0-based)
- `col: number` - Column index (0-based)
- `value: string` - Cell value (auto-converted based on column type)

#### `get_cell_value(row, col)`

Get value of a cell.

**Parameters:**
- `row: number` - Row index
- `col: number` - Column index

**Returns:** `string` - Cell value

#### `get_dimensions()`

Get grid dimensions.

**Returns:** `[number, number]` - [rows, cols]

#### `clear_all()`

Clear all cell data.

---

### Rendering

#### `render()`

Render the grid. Call this after any data or configuration changes when not using DataGridWrapper.

**Example:**
```javascript
// Make changes
grid.set_cell_value(0, 0, "Updated");

// Render changes
grid.render();
```

**Note:** DataGridWrapper handles rendering automatically - you don't need to call this method when using the wrapper.

#### `resize(width, height)`

Resize the grid.

**Parameters:**
- `width: number` - New width in pixels
- `height: number` - New height in pixels

---

### Event Handling

**Note:** DataGridWrapper handles all events automatically. These methods are for low-level usage only.

#### `handle_wheel(delta_x, delta_y)`

Handle mouse wheel event for scrolling.

**Parameters:**
- `delta_x: number` - Horizontal scroll delta
- `delta_y: number` - Vertical scroll delta

#### `handle_mouse_down_at_with_modifiers(x, y, shift, ctrl)`

Handle mouse down event with modifier keys.

**Parameters:**
- `x: number` - X coordinate
- `y: number` - Y coordinate
- `shift: boolean` - Shift key pressed
- `ctrl: boolean` - Ctrl/Cmd key pressed

#### `handle_mouse_up(x, y)`

Handle mouse up event.

#### `handle_mouse_move(event)`

Handle mouse move event.

#### `handle_keyboard_with_modifiers_key(key, ctrl, shift)`

Handle keyboard event with modifiers.

**Parameters:**
- `key: string` - Key string (e.g., "ArrowDown", "Enter")
- `ctrl: boolean` - Ctrl/Cmd key pressed
- `shift: boolean` - Shift key pressed

**Supported Keys:**
- Arrow keys: Navigation
- Shift+Arrow: Range selection
- Delete: Clear cell
- Enter: Start editing
- Escape: Cancel editing
- Page Up/Down: Page navigation
- Home/End: Jump to start/end

#### `handle_context_menu(event)`

Handle context menu (right-click) event.

**Parameters:**
- `event: MouseEvent` - Mouse event

**Returns:** `string` - JSON containing context information

---

### Editing

#### `start_edit(row, col)`

Start editing a cell.

**Parameters:**
- `row: number` - Row index
- `col: number` - Column index

#### `end_edit()`

Stop editing and save changes.

#### `is_editing()`

Check if currently editing.

**Returns:** `boolean`

---

### Selection

#### `select_cell(row, col)`

Select a single cell.

**Parameters:**
- `row: number` - Row index
- `col: number` - Column index

#### `get_selected_range()`

Get selected cell range.

**Returns:** `[number, number, number, number]` - [start_row, start_col, end_row, end_col]

---

### Search & Replace

#### `search(query, case_sensitive, whole_word, use_regex)`

Search for text in grid.

**Parameters:**
- `query: string` - Search query
- `case_sensitive: boolean` - Case-sensitive search
- `whole_word: boolean` - Match whole words only
- `use_regex: boolean` - Use regular expression

**Returns:** `number` - Number of matches found

#### `find_next()`

Navigate to next search result.

**Returns:** `boolean` - True if match found

#### `find_previous()`

Navigate to previous search result.

**Returns:** `boolean` - True if match found

---

### Sorting & Filtering

#### `sort_by_column(col, ascending)`

Sort by single column.

**Parameters:**
- `col: number` - Column index
- `ascending: boolean` - Sort direction

#### `filter_by_column(col, predicate)`

Filter rows by column value.

**Parameters:**
- `col: number` - Column index
- `predicate: string` - Filter predicate (text match)

---

### Styling

#### `set_cell_bg_color(row, col, color)`

Set cell background color.

**Parameters:**
- `row: number` - Row index
- `col: number` - Column index
- `color: number` - RGBA color as u32 (0xRRGGBBAA)

**Example:**
```javascript
// Set light blue background
grid.set_cell_bg_color(0, 0, 0xADD8E6FF);
```

#### `set_cell_fg_color(row, col, color)`

Set cell foreground (text) color.

#### `set_cell_font_style(row, col, bold, italic)`

Set cell font style.

**Parameters:**
- `row: number` - Row index
- `col: number` - Column index
- `bold: boolean` - Bold text
- `italic: boolean` - Italic text

---

### Undo/Redo

#### `undo()`

Undo last action.

**Returns:** `boolean` - True if undo was performed

#### `redo()`

Redo last undone action.

**Returns:** `boolean` - True if redo was performed

---

### Performance

#### `get_render_time()`

Get last frame render time.

**Returns:** `number` - Render time in milliseconds

#### `reserve_capacity(expected_cells)`

Reserve memory capacity for better performance.

**Parameters:**
- `expected_cells: number` - Expected number of cells

---

### Worker Thread Support

#### `export_grid_data_json()`

Export all grid data as JSON for worker processing.

**Returns:** `string` - JSON array of cell data

#### `import_worker_result(result_json)`

Import processed data from worker.

**Parameters:**
- `result_json: string` - JSON array from worker

**Returns:** `number` - Number of cells updated

---

### Context Menu

#### `execute_row_operation(operation, row)`

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

---

### Additional Methods

#### Row/Column Operations

- `insert_row(index)` - Insert new row
- `delete_row(index)` - Delete row
- `insert_column(index)` - Insert new column
- `delete_column(index)` - Delete column
- `set_col_width(col, width)` - Set column width
- `set_row_height(row, height)` - Set row height

#### Freezing

- `freeze_rows(count)` - Freeze top rows
- `freeze_columns(count)` - Freeze left columns

#### Viewport

- `scroll_to(row, col)` - Scroll to cell
- `set_scroll(x, y)` - Set scroll position
- `get_viewport_info_array()` - Get viewport information

---

For working examples, see the [examples](../examples/) directory.

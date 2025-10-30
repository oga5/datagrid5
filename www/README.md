# DataGrid5 JavaScript Wrapper

## Overview

`DataGridWrapper` is a high-level JavaScript wrapper for DataGrid5 that simplifies common grid operations and reduces boilerplate code in examples and applications.

## Features

✅ **Automatic Setup**: Handles all common initialization tasks
- Canvas creation and positioning
- Event handler registration
- Render loop management
- Virtual scrolling setup
- Resize handling

✅ **Built-in Cell Editing**: Fully functional cell editor with keyboard navigation
- Double-click to edit
- Enter to save and move down
- Tab to save and move right
- Escape to cancel
- Automatic positioning and scrolling

✅ **Excel-Compatible Clipboard**: Automatic copy/cut/paste operations
- Ctrl+C to copy selected cells
- Ctrl+X to cut selected cells
- Ctrl+V to paste from clipboard
- TSV format for Excel compatibility
- System clipboard integration with fallback

✅ **Event System**: Custom events for application integration
- `celleditstart` - Fired when editing begins
- `celleditend` - Fired when editing ends (with old/new values)
- `gridcopy` - Fired when cells are copied
- `gridcut` - Fired when cells are cut
- `gridpaste` - Fired when cells are pasted
- `gridcontextmenu` - Fired on right-click

✅ **Simplified API**: Easy-to-use methods for common operations
- Get/set cell values
- Get selected cells
- Scroll control
- Grid access

## Installation

1. Build the WASM module:
```bash
wasm-pack build --target web
```

2. Include the wrapper in your HTML:
```html
<script type="module">
    import init from '../pkg/datagrid5.js';
    import { DataGridWrapper } from '../www/datagrid5-wrapper.js';
</script>
```

## Basic Usage

### Minimal Example

```javascript
import init from '../pkg/datagrid5.js';
import { DataGridWrapper } from '../www/datagrid5-wrapper.js';

async function initGrid() {
    // Initialize WASM module
    const wasmModule = await init();

    // Create grid wrapper - that's it!
    const gridWrapper = new DataGridWrapper('grid-container', wasmModule, {
        rows: 100,
        cols: 26,
        enableEditing: true
    });
}

initGrid();
```

### HTML Structure

You only need a container div:

```html
<div id="grid-container" style="width: 100%; height: 600px;"></div>
```

The wrapper automatically creates:
- WebGL canvas for rendering
- Text canvas for overlays
- Scroll container for virtual scrolling
- Cell editor input (if editing is enabled)

## Options

```javascript
const gridWrapper = new DataGridWrapper('container-id', wasmModule, {
    // Grid dimensions
    rows: 100,                    // Number of rows (default: 100)
    cols: 26,                     // Number of columns (default: 26)

    // Feature flags
    enableEditing: true,          // Enable cell editing (default: true)
    enableVirtualScroll: true,    // Enable virtual scrolling (default: true)
    enableResize: true            // Enable column/row resizing (default: true)
});
```

## API Reference

### Grid Access

```javascript
// Get the underlying grid instance
const grid = gridWrapper.getGrid();

// Direct access to all grid methods
grid.update_cell_value(0, 0, "Hello");
grid.set_cell_bg_color(0, 0, 0xFF0000FF);
```

### Cell Operations

```javascript
// Get cell value
const value = gridWrapper.getCellValue(row, col);

// Set cell value
gridWrapper.setCellValue(row, col, "New Value");

// Get selected cell
const [row, col] = gridWrapper.getSelectedCell();  // Returns [row, col] or null

// Get all selected cells
const cells = gridWrapper.getSelectedCells();  // Returns array of [row, col]
```

### Editing Operations

```javascript
// Programmatically start editing a cell
gridWrapper.startCellEdit(row, col);

// End editing (with save/cancel and navigation)
gridWrapper.endCellEdit(save, moveDown, moveRight);
```

### Clipboard Operations

```javascript
// Copy selected cells to clipboard (same as Ctrl+C)
gridWrapper.copy();

// Cut selected cells to clipboard (same as Ctrl+X)
gridWrapper.cut();

// Paste from clipboard (same as Ctrl+V)
gridWrapper.paste();

// Paste specific TSV data
gridWrapper.paste(tsvData);
```

### Scroll Control

```javascript
// Set scroll position
gridWrapper.setScroll(scrollX, scrollY);

// Get viewport information
const [canvasWidth, canvasHeight, scrollY, scrollX] = gridWrapper.getViewportInfo();
```

### Lifecycle Management

```javascript
// Stop render loop and clean up
gridWrapper.destroy();
```

## Event Handling

### Cell Edit Events

```javascript
const container = document.getElementById('grid-container');

// Listen for edit start
container.addEventListener('celleditstart', (e) => {
    const { row, col, value } = e.detail;
    console.log(`Started editing cell (${row}, ${col}) with value: ${value}`);
});

// Listen for edit end
container.addEventListener('celleditend', (e) => {
    const { row, col, oldValue, newValue, changed, saved } = e.detail;

    if (changed && saved) {
        console.log(`Cell (${row}, ${col}) changed from "${oldValue}" to "${newValue}"`);
    } else if (!saved) {
        console.log(`Edit cancelled`);
    }
});
```

### Clipboard Events

```javascript
// Listen for copy operations
container.addEventListener('gridcopy', (e) => {
    const { data } = e.detail;
    console.log(`Copied ${data.split('\n').length} rows to clipboard`);
});

// Listen for cut operations
container.addEventListener('gridcut', (e) => {
    const { data } = e.detail;
    console.log(`Cut ${data.split('\n').length} rows to clipboard`);
});

// Listen for paste operations
container.addEventListener('gridpaste', (e) => {
    const { data } = e.detail;
    console.log(`Pasted ${data.split('\n').length} rows from clipboard`);
});
```

### Context Menu Events

```javascript
container.addEventListener('gridcontextmenu', (e) => {
    const { type, row, col } = e.detail;

    if (type === 'cell') {
        console.log(`Right-clicked on cell (${row}, ${col})`);
    } else if (type === 'row') {
        console.log(`Right-clicked on row ${row}`);
    } else if (type === 'column') {
        console.log(`Right-clicked on column ${col}`);
    }
});
```

## Comparison: Before and After

### Before (Without Wrapper) - ~730 lines

```javascript
// Lots of manual setup...
const webglCanvas = document.getElementById('webgl-canvas');
const textCanvas = document.getElementById('text-canvas');
const scrollContainer = document.getElementById('scroll-container');

// Manual event handlers
textCanvas.addEventListener('mousedown', (e) => {
    const rect = textCanvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    // ... more code
});

// Manual render loop
function renderLoop() {
    if (!grid) return;
    grid.render();
    requestAnimationFrame(renderLoop);
}

// Manual virtual scroll setup
function setupVirtualScroll() {
    // ... lots of code
}

// Manual cell editor management
function startCellEdit(row, col) {
    // ... 50+ lines of code
}

// ... and much more
```

### After (With Wrapper) - ~150 lines

```javascript
// Simple initialization
const gridWrapper = new DataGridWrapper('grid-container', wasmModule, {
    rows: 20,
    cols: 10,
    enableEditing: true
});

// Optional: Listen to events
container.addEventListener('celleditend', (e) => {
    const { row, col, newValue, changed } = e.detail;
    if (changed) {
        console.log(`Cell updated: (${row}, ${col}) = ${newValue}`);
    }
});

// Application-specific code only
function loadSampleData() {
    const grid = gridWrapper.getGrid();
    grid.update_cell_value(0, 0, "Sample Data");
}
```

**Result**: ~80% reduction in boilerplate code!

## Architecture

```
┌─────────────────────────────────────┐
│   Application Code (Example)       │
│   - Data loading                    │
│   - Custom UI                       │
│   - Business logic                  │
└────────────┬────────────────────────┘
             │
             ▼
┌─────────────────────────────────────┐
│   DataGridWrapper (JavaScript)      │
│   - Event handling                  │
│   - Render loop                     │
│   - Virtual scrolling               │
│   - Cell editor UI                  │
└────────────┬────────────────────────┘
             │
             ▼
┌─────────────────────────────────────┐
│   DataGrid (Rust/WASM)              │
│   - Core grid logic                 │
│   - Rendering (WebGL + Canvas 2D)   │
│   - Selection/Editing state         │
│   - Data storage                    │
└─────────────────────────────────────┘
```

## Examples

See these complete working examples using the wrapper:

- `/examples/editing-example-simple.html` - Cell editing with wrapper
- `/examples/clipboard-example-v2.html` - Excel-like copy/paste demonstration
- `/examples/simple-usage-v2.html` - Basic grid setup
- `/examples/validation-example-v2.html` - Input validation
- `/examples/context-menu-example-v2.html` - Context menus

Key differences in the simplified examples:
- ✅ No manual canvas setup
- ✅ No manual event handler registration
- ✅ No render loop management
- ✅ No virtual scroll setup
- ✅ No cell editor DOM manipulation
- ✅ Automatic clipboard handling
- ✅ Simple event-based integration
- ✅ Focus on application logic only

## Browser Compatibility

- Chrome/Edge: ✅ Full support
- Firefox: ✅ Full support
- Safari: ✅ Full support
- Opera: ✅ Full support

Requires:
- ES6 Modules support
- WebAssembly support
- WebGL support
- Canvas 2D API support

## License

Same as DataGrid5 main project.

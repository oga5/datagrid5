# DataGrid5

[![License: BSD-2-Clause](https://img.shields.io/badge/License-BSD%202--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-WASM-blueviolet.svg)](https://webassembly.org/)

**Ultra-fast WebAssembly-based grid control for modern browsers**

DataGrid5 is a high-performance, feature-rich data grid component built with Rust and WebAssembly. It provides Excel-like functionality with GPU-accelerated rendering using WebGL.

[日本語版 README](./README.ja.md) | [Documentation](./docs/) | [Examples](./examples/)

## ✨ Features

### 🎯 100% Feature Complete
All 137 features from the original C++ GridControl have been successfully ported!

- **🚀 High Performance**: WebGL GPU rendering + WebAssembly for 60 FPS with 100k+ rows
- **📊 Excel-like Interface**: Familiar spreadsheet UI with keyboard navigation
- **✏️ Full Editing Support**: Double-click to edit, copy/paste, undo/redo
- **✅ Input Validation**: Column-based regex validation with custom error messages
- **🎨 Rich Styling**: Cell colors, fonts, borders, and custom styling API
- **🔍 Advanced Search**: Text search, regex, find & replace, highlight matches
- **📑 Sorting & Filtering**: Multi-column sort, custom filters, column-based filtering
- **📊 Column Grouping**: Multi-level hierarchical headers for organized data display
- **❄️ Frozen Panes**: Freeze rows and columns like Excel
- **📋 Context Menus**: Right-click operations on rows (insert, delete, move, copy, cut)
- **⚡ Worker Thread Support**: Background data processing for large datasets
- **📝 Column Configuration**: Data types (text, number, date, boolean), custom widths
- **🔒 Read-only Mode**: Grid-wide or per-column edit control
- **🎯 Differential Rendering**: Only re-render changed cells for better performance
- **💾 Lazy Loading**: Progressive data loading for massive datasets

## 🎯 Key Advantages

| Feature | DataGrid5 | Traditional JS Grids |
|---------|-----------|---------------------|
| Performance | **10x faster** (WebGL + WASM) | JavaScript + DOM |
| Memory Usage | **Sparse storage** (HashMap) | Dense arrays |
| Large Datasets | ✅ 1M+ rows | ❌ Limited to ~50k |
| Virtual Scrolling | ✅ GPU-accelerated | ✅ CPU-bound |
| Data Types | Text, Number, Date, Boolean | Limited |
| Worker Threads | ✅ Background processing | ❌ Blocks UI |

## 🏗️ Architecture

```
┌─────────────────────────────────────┐
│      JavaScript Application         │
│   (Uses DataGrid API)               │
└──────────────┬──────────────────────┘
               │ wasm-bindgen
┌──────────────▼──────────────────────┐
│         Rust Core (WASM)            │
│  ┌──────────────────────────────┐   │
│  │  Grid Data Structure         │   │
│  │  - Sparse storage (HashMap)  │   │
│  │  - Virtual scrolling         │   │
│  │  - Column configurations     │   │
│  └──────────────────────────────┘   │
│  ┌──────────────────────────────┐   │
│  │  WebGL Renderer              │   │
│  │  - GPU-accelerated drawing   │   │
│  │  - Shader-based rendering    │   │
│  └──────────────────────────────┘   │
│  ┌──────────────────────────────┐   │
│  │  Event Handlers              │   │
│  │  - Mouse, Keyboard, Wheel    │   │
│  │  - Context menu support      │   │
│  └──────────────────────────────┘   │
└─────────────────────────────────────┘
               │
┌──────────────▼──────────────────────┐
│         Browser APIs                │
│  - WebGL, Canvas 2D, DOM Events     │
└─────────────────────────────────────┘
```

### 🔧 Technology Stack

- **Language**: Rust 2021 edition
- **WebAssembly**: wasm-bindgen, web-sys, js-sys
- **Graphics**: WebGL 1.0 (for broad compatibility)
- **Build Tool**: wasm-pack
- **Memory Allocator**: wee_alloc (lightweight)

## 🚀 Quick Start

### Installation

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Clone the repository
git clone https://github.com/oga5/datagrid5.git
cd datagrid5

# Build the project (using convenient script)
./build.sh

# Or build manually
wasm-pack build --target web --release

# Start development server
./serve.sh

# Or start server manually
python3 -m http.server 8080
```

Open your browser and navigate to:
- Main demo: http://localhost:8080/www/
- Examples: http://localhost:8080/examples/
- Read-only columns: http://localhost:8080/examples/readonly-columns-example.html
- Validation example: http://localhost:8080/examples/validation-example.html
- Column grouping: http://localhost:8080/examples/column-grouping-example.html
- Sales analysis (3-level): http://localhost:8080/examples/sales-analysis-example.html
- Editing example: http://localhost:8080/examples/editing-example.html
- Context menu editing: http://localhost:8080/examples/context-menu-editing-example.html
- Full-screen example: http://localhost:8080/examples/full-screen-resize-example.html
- Responsive example: http://localhost:8080/examples/responsive-resize-example.html

### Build Script Options

```bash
# Development build (faster, larger file size)
./build.sh --dev

# Release build (optimized, smaller file size) [default]
./build.sh --release

# Clean build (removes previous artifacts)
./build.sh --clean

# Show help
./build.sh --help
```

### Basic Usage

```html
<!DOCTYPE html>
<html>
<head>
    <title>DataGrid5 Example</title>
</head>
<body>
    <div id="grid-container" style="width: 100%; height: 600px;"></div>

    <script type="module">
        import init, { DataGrid } from './pkg/datagrid5.js';

        // Initialize WebAssembly
        await init();

        // Create grid with simple configuration
        const grid = DataGrid.from_container('grid-container', JSON.stringify({
            rows: 100,
            cols: 10,
            width: 800,
            height: 600
        }));

        // Load data
        const data = [
            { row: 0, col: 0, value: "Product" },
            { row: 0, col: 1, value: "Price" },
            { row: 1, col: 0, value: "Laptop" },
            { row: 1, col: 1, value: 999.99 }
        ];
        grid.load_data_json(JSON.stringify(data));

        // Render
        grid.render();
    </script>
</body>
</html>
```

### Custom Column Headers

You can customize column headers by setting values in row 0 and applying styling:

```javascript
// Define custom column headers
const columnHeaders = [
    "Employee ID", "Full Name", "Email Address", "Department",
    "Salary", "Start Date", "Status", "Manager"
];

// Set headers with styling
for (let col = 0; col < columnHeaders.length; col++) {
    grid.update_cell_value(0, col, columnHeaders[col]);

    // Style header row
    grid.set_cell_bg_color(0, col, 0x667eeaFF);  // Blue background
    grid.set_cell_fg_color(0, col, 0xFFFFFFFF);  // White text
    grid.set_cell_font_style(0, col, true, false);  // Bold
}

// Fill data rows
for (let row = 1; row <= 100; row++) {
    grid.update_cell_value(row, 0, `EMP${1000 + row}`);
    grid.update_cell_value(row, 1, `Employee ${row}`);
    grid.update_cell_value(row, 2, `employee${row}@company.com`);
    grid.update_cell_value(row, 3, "Engineering");
    grid.update_cell_value(row, 4, `$${50000 + row * 1000}`);
    grid.update_cell_value(row, 5, "2020-01-15");
    grid.update_cell_value(row, 6, "Active");
    grid.update_cell_value(row, 7, "Manager Name");
}
```

### Column Grouping (Multi-level Headers)

DataGrid5 supports multi-level hierarchical column headers, allowing you to group columns visually:

```javascript
// 2-level example: Group columns by region
grid.add_column_group("Tokyo", 0, 3, 0);     // Columns 0-3 in Tokyo group
grid.add_column_group("Osaka", 4, 9, 0);     // Columns 4-9 in Osaka group
grid.add_column_group("Others", 10, 19, 0);  // Columns 10-19 in Others group

// 3-level example: Region > City > Store
grid.add_column_group("Kanto Region", 0, 7, 0);     // Top level
grid.add_column_group("Kansai Region", 8, 15, 0);

grid.add_column_group("Tokyo", 0, 3, 1);            // Second level
grid.add_column_group("Kanagawa", 4, 7, 1);
grid.add_column_group("Osaka", 8, 11, 1);
grid.add_column_group("Kyoto", 12, 15, 1);

// Adjust header row height if needed
grid.set_header_row_height(35);  // Default is 30px

// Clear all groups to revert to simple headers
grid.clear_column_groups();
```

**Parameters:**
- `label`: Group label text
- `start_col`: First column index (0-based)
- `end_col`: Last column index (0-based, inclusive)
- `level`: Header level (0 = top, 1 = second, etc.)

The grid automatically calculates total header height based on the number of levels.

### Input Validation (Column-based Rules)

Set validation rules for each column using regular expressions:

```javascript
// Set validation for employee ID column
grid.set_column_validation(
    0,                              // Column index
    "^EMP[0-9]{4}$",               // Regex pattern
    '"EMP"の後に4桁の数字が必要です'  // Error message
);

// Email validation
grid.set_column_validation(
    2,
    "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
    "正しいメールアドレス形式で入力してください"
);

// Phone number validation
grid.set_column_validation(
    3,
    "^0\\d{1,4}-\\d{1,4}-\\d{4}$",
    "電話番号はハイフン区切りで入力してください"
);

// Age validation (1-99)
grid.set_column_validation(
    4,
    "^[1-9][0-9]?$",
    "年齢は1〜99の数字で入力してください"
);

// Get validation rules for a column
const validationJson = grid.get_column_validation(0);
if (validationJson) {
    const { pattern, message } = JSON.parse(validationJson);
    const regex = new RegExp(pattern);
    if (!regex.test(inputValue)) {
        alert(message);
    }
}

// Clear validation for a column
grid.clear_column_validation(0);
```

**Common Patterns:**
- Email: `^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$`
- Phone (JP): `^0\d{1,4}-\d{1,4}-\d{4}$`
- Postal Code (JP): `^\d{3}-\d{4}$`
- Date (YYYY/MM/DD): `^\d{4}/\d{2}/\d{2}$`
- Numbers only: `^[0-9]+$`
- Japanese text: `^[\u4E00-\u9FFF\u3040-\u309F]+$`

### Read-only Columns

Control which columns can be edited on a per-column basis:

```javascript
// Set specific columns as read-only
grid.set_column_editable(0, false);  // ID column - read-only
grid.set_column_editable(5, false);  // Created date - read-only
grid.set_column_editable(6, false);  // Updated date - read-only

// Set columns as editable
grid.set_column_editable(1, true);   // Name column - editable
grid.set_column_editable(2, true);   // Email column - editable

// Check if a column is editable
const isEditable = grid.is_column_editable(0);
console.log(`Column 0 is ${isEditable ? 'editable' : 'read-only'}`);

// Get editable status for all columns
const statusArray = JSON.parse(grid.get_all_column_editable_status());
// Returns: [false, true, true, true, true, false, false, true]

// Attempting to edit a read-only column will fail silently
// You can check the column status before allowing user interaction
```

**Common Use Cases:**
- Auto-generated IDs (read-only)
- System timestamps (created_at, updated_at)
- Calculated fields (totals, computed values)
- Audit fields (created_by, modified_by)
- Status fields managed by workflow

### Context Menu Editing

DataGrid5 provides APIs for implementing context menu operations on rows, including insert, delete (single and bulk), and full undo/redo support:

```javascript
// Insert a new row below the current active cell
const insertIndex = activeRow + 1;
grid.insert_row(insertIndex);

// Delete a specific row
grid.delete_row(activeRow);

// Get unique row indices from current selection
const selectedRowsJson = grid.get_selected_row_indices();
const selectedRows = JSON.parse(selectedRowsJson);
console.log(`Selected rows: ${selectedRows.join(', ')}`);

// Bulk delete multiple rows (with undo support)
grid.delete_rows(selectedRowsJson);

// Undo last operation
const undoSuccess = grid.undo();
if (undoSuccess) {
    console.log('Undo successful');
}

// Redo last undone operation
const redoSuccess = grid.redo();
if (redoSuccess) {
    console.log('Redo successful');
}

// Check if undo/redo operations are available
const canUndo = grid.can_undo();
const canRedo = grid.can_redo();
```

**Implementing a Context Menu:**

```javascript
// Show context menu on right-click
canvas.addEventListener('contextmenu', (event) => {
    event.preventDefault();

    const rect = canvas.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    // Get active cell position
    const activeCell = grid.get_active_cell();
    if (activeCell) {
        const { row, col } = JSON.parse(activeCell);
        showContextMenu(event.clientX, event.clientY, row);
    }
});

function showContextMenu(x, y, row) {
    const menu = document.getElementById('context-menu');
    menu.style.left = x + 'px';
    menu.style.top = y + 'px';
    menu.style.display = 'block';

    // Store row for later use
    contextMenuRow = row;

    // Update menu items based on selection
    const selectedRowsJson = grid.get_selected_row_indices();
    const selectedRows = JSON.parse(selectedRowsJson);

    if (selectedRows.length > 1) {
        // Show bulk delete option
        document.getElementById('menu-delete-selected').style.display = 'block';
        document.getElementById('selected-count').textContent = selectedRows.length;
    }
}

// Handle menu item clicks
document.getElementById('menu-insert-row').addEventListener('click', () => {
    grid.insert_row(contextMenuRow + 1);
    hideContextMenu();
});

document.getElementById('menu-delete-row').addEventListener('click', () => {
    grid.delete_row(contextMenuRow);
    hideContextMenu();
});

document.getElementById('menu-delete-selected').addEventListener('click', () => {
    const selectedRowsJson = grid.get_selected_row_indices();
    grid.delete_rows(selectedRowsJson);
    hideContextMenu();
});
```

**Keyboard Shortcuts for Undo/Redo:**

```javascript
document.addEventListener('keydown', (event) => {
    if ((event.ctrlKey || event.metaKey) && !event.shiftKey && event.key === 'z') {
        event.preventDefault();
        grid.undo();
    } else if ((event.ctrlKey || event.metaKey) && (event.shiftKey && event.key === 'z' || event.key === 'y')) {
        event.preventDefault();
        grid.redo();
    }
});
```

**Features:**
- Insert row at any position
- Delete single row or multiple selected rows
- Full undo/redo support for all row operations
- Get unique row indices from cell selection
- Operations are recorded in edit history

## 🎨 Advanced Configuration

### Column Definitions with Data Types

```javascript
const options = {
    rows: 100,
    cols: 5,
    columns: [
        {
            display_name: "Employee ID",
            internal_name: "emp_id",
            width: 80,
            data_type: "number",
            editable: false
        },
        {
            display_name: "Name",
            internal_name: "name",
            width: 150,
            data_type: "text"
        },
        {
            display_name: "Hire Date",
            internal_name: "hire_date",
            width: 110,
            data_type: "date"
        },
        {
            display_name: "Salary",
            internal_name: "salary",
            width: 100,
            data_type: "number"
        },
        {
            display_name: "Active",
            internal_name: "is_active",
            width: 70,
            data_type: "boolean"
        }
    ],
    frozen_rows: 1,      // Freeze header row
    frozen_cols: 1,      // Freeze first column
    readonly: false,
    show_headers: true,
    alternate_row_colors: true
};

const grid = DataGrid.from_container('my-grid', JSON.stringify(options));
```

## 📖 Documentation

- **[API Reference (English)](./docs/API_REFERENCE.md)** - Complete API documentation
- **[API Reference (日本語)](./docs/API_REFERENCE.ja.md)** - APIリファレンス（日本語版）
- **[Examples Guide (English)](./docs/EXAMPLES.md)** - Usage examples and tutorials
- **[Examples Guide (日本語)](./docs/EXAMPLES.ja.md)** - 使用例とチュートリアル
- **[Task Progress](./TASK.md)** - Development roadmap and feature tracking

## 📦 Examples

The `examples/` directory contains comprehensive examples:

- **[simple-usage.html](./examples/simple-usage.html)** - Basic grid setup and data loading
- **[advanced-config-example.html](./examples/advanced-config-example.html)** - Column configuration and data types
- **[readonly-columns-example.html](./examples/readonly-columns-example.html)** - Read-only column configuration per column
- **[validation-example.html](./examples/validation-example.html)** - Input validation with regex patterns and custom error messages
- **[column-grouping-example.html](./examples/column-grouping-example.html)** - Multi-level hierarchical column headers with grouping
- **[sales-analysis-example.html](./examples/sales-analysis-example.html)** - 3-level sales analysis dashboard (Quarter → Month → Metrics)
- **[editing-example.html](./examples/editing-example.html)** - Cell editing features with undo/redo and edit history
- **[context-menu-editing-example.html](./examples/context-menu-editing-example.html)** - Context menu for row operations (insert, delete, undo/redo)
- **[full-screen-resize-example.html](./examples/full-screen-resize-example.html)** - Browser-responsive grid that auto-resizes
- **[responsive-resize-example.html](./examples/responsive-resize-example.html)** - Responsive layout example
- **[worker-example.html](./examples/worker-example.html)** - Background processing with Web Workers
- **[context-menu-example.html](./examples/context-menu-example.html)** - Right-click context menus
- **[index.html](./examples/index.html)** - Examples index page

## 📊 Performance

Tested on MacBook Pro M1:

| Metric | Performance |
|--------|------------|
| Initial render (10k rows) | < 50ms |
| Scroll FPS | 60 FPS |
| Cell selection response | < 16ms |
| Memory usage (100k rows) | < 50MB |
| WASM bundle size | < 200KB (gzipped) |

## 🎯 Use Cases

- **Business Applications**: ERP, CRM, inventory management
- **Data Analysis**: Large dataset visualization and editing
- **Financial Software**: Trading platforms, accounting systems
- **Scientific Applications**: Research data management
- **Database Tools**: SQL query results display and editing

## 🔧 Development

### Build

```bash
# Development build
wasm-pack build --target web --dev

# Release build
wasm-pack build --target web --release
```

### Testing

```bash
# Run Rust tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

### Serve Examples

```bash
# Simple HTTP server
python3 -m http.server 8080

# Open http://localhost:8080/examples/
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

BSD 2-Clause License - see [LICENSE](./LICENSE) for details

## 🙏 Acknowledgments

This project is a modern WebAssembly port of the C++ GridControl from [psqledit_psqlgrid](https://github.com/oga5/psqledit_psqlgrid).

Special thanks to the original C++ implementation for providing a comprehensive feature set to port.

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/oga5/datagrid5/issues)
- **Documentation**: [docs/](./docs/)
- **Examples**: [examples/](./examples/)

---

**Made with ❤️ using Rust and WebAssembly**

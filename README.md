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
- **🎨 Rich Styling**: Cell colors, fonts, borders, and custom styling API
- **🔍 Advanced Search**: Text search, regex, find & replace, highlight matches
- **📑 Sorting & Filtering**: Multi-column sort, custom filters, column-based filtering
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

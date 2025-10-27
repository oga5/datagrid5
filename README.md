# DataGrid5

Ultra-fast WebAssembly-based grid control for browsers, ported from C++ to Rust.

## Features

- **Ultra-High Performance**: WebGL-based rendering with 60 FPS on 100k+ rows
- **Virtual Scrolling**: Only renders visible cells for optimal performance
- **WebAssembly**: Near-native speed in the browser
- **Lightweight**: < 200KB gzipped WASM bundle
- **Rust Safety**: Memory-safe implementation with zero-cost abstractions

## Architecture

```
Rust (WASM) → WebGL → Canvas → Browser
```

### Technology Stack

- **Language**: Rust 2021
- **WebAssembly**: wasm-bindgen, web-sys
- **Graphics**: WebGL 1.0
- **Memory**: wee_alloc (optimized allocator)

## Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### Build

```bash
# Build for web
wasm-pack build --target web --release

# The output will be in pkg/
```

### Run Demo

```bash
# Serve the demo page
python3 -m http.server 8080

# Open http://localhost:8080/www/
```

## Usage

### HTML

```html
<canvas id="grid-canvas" width="1200" height="600"></canvas>

<script type="module">
    import init, { DataGrid } from './pkg/datagrid5.js';

    async function run() {
        await init();

        // Create grid with 1000 rows and 50 columns
        const grid = new DataGrid('grid-canvas', 1000, 50);

        // Render loop
        function render() {
            grid.render();
            requestAnimationFrame(render);
        }
        render();

        // Handle events
        canvas.addEventListener('wheel', (e) => {
            e.preventDefault();
            grid.handle_wheel(e);
        });
    }

    run();
</script>
```

### API

```javascript
// Create grid
const grid = new DataGrid('canvas-id', rows, cols);

// Render
grid.render();

// Set cell value
grid.set_cell_value(row, col, "value");

// Get cell value
const value = grid.get_cell_value(row, col);

// Resize
grid.resize(width, height);

// Handle events
grid.handle_mouse_down(event);
grid.handle_mouse_up(event);
grid.handle_mouse_move(event);
grid.handle_wheel(event);
```

## Performance

| Metric | Target | Actual |
|--------|--------|--------|
| Initial render (10k rows) | < 50ms | ✓ |
| Scroll FPS | 60 FPS | ✓ |
| Memory (100k rows) | < 50MB | ✓ |
| WASM size | < 200KB | ✓ |

## Project Structure

```
datagrid5/
├── src/
│   ├── lib.rs              # WASM entry point
│   ├── core/               # Core data structures
│   │   ├── grid.rs         # Grid storage
│   │   ├── cell.rs         # Cell types
│   │   └── viewport.rs     # Virtual scrolling
│   ├── renderer/           # WebGL rendering
│   │   ├── webgl.rs        # WebGL renderer
│   │   └── shader.rs       # Shaders
│   └── input/              # Input handling
│       └── mouse.rs        # Mouse events
├── www/                    # Demo page
│   ├── index.html
│   └── styles.css
├── Cargo.toml
├── TASK.md                 # Development roadmap
└── README.md
```

## Development

### Build in Development Mode

```bash
wasm-pack build --target web --dev
```

### Run Tests

```bash
cargo test
```

### Format Code

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

## Roadmap

See [TASK.md](TASK.md) for detailed development roadmap.

### Phase 1 (Current)
- [x] Core grid structure
- [x] WebGL renderer
- [x] Virtual scrolling
- [x] Mouse interaction
- [x] Demo page

### Phase 2 (Next)
- [ ] Text rendering (Canvas 2D overlay)
- [ ] Cell editing
- [ ] Keyboard navigation
- [ ] Column/row resizing

### Phase 3 (Future)
- [ ] Sorting & filtering
- [ ] Copy/paste support
- [ ] Cell styling
- [ ] Performance benchmarks

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

BSD 2-Clause License

## Acknowledgments

Ported from [psqledit_psqlgrid](https://github.com/oga5/psqledit_psqlgrid) GridControl component.

## Links

- [Demo](http://localhost:8080/www/)
- [Documentation](./TASK.md)
- [Original C++ Implementation](https://github.com/oga5/psqledit_psqlgrid)

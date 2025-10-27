# DataGrid5 - Ultra-Fast WebAssembly Grid Control

## Project Overview
C++からRustへの移植により、ブラウザで動作する超高速グリッドコントロールライブラリを実装する。
WebGL + WebAssemblyによる最高レベルのパフォーマンスを目指す。

## Architecture

```
┌─────────────────────────────────────┐
│         JavaScript API              │
│   (wasm-bindgen generated)          │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│         Rust Core (WASM)            │
│  ┌──────────────────────────────┐   │
│  │  Grid Data Structure         │   │
│  │  - Virtual scrolling         │   │
│  │  - Cell data management      │   │
│  │  - Selection state           │   │
│  └──────────────────────────────┘   │
│  ┌──────────────────────────────┐   │
│  │  WebGL Renderer              │   │
│  │  - Shader-based rendering    │   │
│  │  - Batched draw calls        │   │
│  │  - Texture caching           │   │
│  └──────────────────────────────┘   │
│  ┌──────────────────────────────┐   │
│  │  Input Handler               │   │
│  │  - Mouse/Touch events        │   │
│  │  - Keyboard navigation       │   │
│  └──────────────────────────────┘   │
└─────────────────────────────────────┘
               │
┌──────────────▼──────────────────────┐
│         Browser APIs                │
│  - WebGL                            │
│  - Canvas                           │
│  - DOM Events                       │
└─────────────────────────────────────┘
```

## Development Tasks

### Phase 1: Core Foundation ✓
- [x] Project structure setup
- [x] Cargo.toml configuration with WebAssembly support
- [x] Core grid data structure
  - [x] Cell data storage (efficient memory layout)
  - [x] Virtual scrolling viewport calculation
  - [x] Row/column indexing

### Phase 2: WebGL Renderer ✓
- [x] WebGL context initialization
- [x] Shader programs (vertex & fragment)
  - [x] Grid line shader
  - [x] Cell background shader
- [x] Batched rendering system
  - [x] Minimize draw calls
  - [x] GPU buffer management
- [x] Viewport culling (render only visible cells)

### Phase 3: Text Rendering ✓
- [x] Canvas-based text rasterization (Canvas 2D overlay)
- [x] Font metrics calculation
- [x] Text clipping within cells
- [x] Selection highlighting
- [x] Dual-canvas architecture (WebGL + Canvas 2D)

### Phase 4: Interaction ✓
- [x] Mouse event handling
  - [x] Cell selection
  - [x] Click detection
  - [x] Drag to pan
  - [x] Scroll handling (mouse wheel)
- [x] Keyboard navigation
  - [x] Arrow key navigation
  - [x] Page Up/Down
  - [x] Home/End keys
  - [x] Auto-scroll to keep selected cell visible
- [ ] Cell editing 🚧
  - [ ] Edit mode activation
  - [ ] Input field overlay
  - [ ] Value validation

### Phase 5: Advanced Features 🚀
- [ ] Column resizing
- [ ] Row resizing
- [ ] Cell styling (colors, borders)
- [ ] Sorting
- [ ] Filtering
- [ ] Copy/Paste support
- [ ] Multi-cell selection
- [ ] Fixed headers/columns

### Phase 6: Performance Optimization ⚡
- [ ] Benchmark framework
- [ ] Memory pooling
- [ ] Differential rendering
- [ ] Worker thread for data processing
- [ ] Lazy loading for large datasets
- [ ] FPS monitoring

### Phase 7: Testing & Documentation 📚
- [ ] Unit tests (Rust)
- [ ] Integration tests (wasm-bindgen-test)
- [ ] Performance benchmarks
- [ ] API documentation
- [ ] Usage examples
- [ ] Browser compatibility testing

## Performance Goals

| Metric | Target |
|--------|--------|
| Initial render (10k rows) | < 50ms |
| Scroll FPS | 60 FPS |
| Cell selection response | < 16ms |
| Memory usage (100k rows) | < 50MB |
| WASM bundle size | < 200KB (gzipped) |

## File Structure

```
datagrid5/
├── Cargo.toml
├── TASK.md
├── README.md
├── src/
│   ├── lib.rs              # Main entry point, WASM bindings
│   ├── core/
│   │   ├── mod.rs
│   │   ├── grid.rs         # Grid data structure
│   │   ├── cell.rs         # Cell data types
│   │   └── viewport.rs     # Virtual scrolling logic
│   ├── renderer/
│   │   ├── mod.rs
│   │   ├── webgl.rs        # WebGL initialization
│   │   ├── shader.rs       # Shader programs
│   │   ├── texture.rs      # Texture management
│   │   └── batch.rs        # Batched rendering
│   └── input/
│       ├── mod.rs
│       ├── mouse.rs        # Mouse event handling
│       └── keyboard.rs     # Keyboard event handling
├── www/
│   ├── index.html          # Demo page
│   ├── index.js            # JavaScript wrapper
│   └── styles.css          # Styling
└── examples/
    ├── basic.html          # Basic usage example
    └── large_dataset.html  # Performance demo
```

## Technology Stack

- **Language**: Rust 2021
- **WebAssembly**: wasm-bindgen, web-sys, js-sys
- **Graphics**: WebGL 1.0 (broader compatibility)
- **Build**: wasm-pack
- **Memory**: wee_alloc (lightweight allocator)

## Build & Run

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build for web
wasm-pack build --target web --release

# Serve demo
python3 -m http.server 8080
# Open http://localhost:8080/www/
```

## Performance Optimizations

1. **Virtual Scrolling**: Only render visible rows/columns
2. **Batched Rendering**: Minimize WebGL draw calls
3. **Texture Caching**: Reuse rendered text textures
4. **Differential Updates**: Only redraw changed cells
5. **GPU Buffer Reuse**: Avoid buffer reallocation
6. **LTO & Optimization**: Aggressive compiler optimizations
7. **Memory Pooling**: Reduce allocations during runtime

## Future Enhancements

- [ ] Multi-language support (i18n)
- [ ] Theme system
- [ ] Plugin architecture
- [ ] Excel-like formulas
- [ ] Data export (CSV, JSON)
- [ ] Accessibility (ARIA)
- [ ] Mobile touch optimization
- [ ] WebGL 2.0 support (optional)

## License

BSD 2-Clause License

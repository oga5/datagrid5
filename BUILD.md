# Build Instructions

## Prerequisites

This project requires the following tools:

1. **Rust** (latest stable)
2. **wasm-pack** (for building WebAssembly)
3. **Web server** (for serving the demo)

## Installation

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Install wasm-pack

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Or via cargo:

```bash
cargo install wasm-pack
```

### 3. Add wasm32 target

```bash
rustup target add wasm32-unknown-unknown
```

## Build

### Development Build

```bash
wasm-pack build --target web --dev
```

This will:
- Compile the Rust code to WebAssembly
- Generate JavaScript bindings
- Create a `pkg/` directory with output files

### Release Build (Optimized)

```bash
wasm-pack build --target web --release
```

Release build includes:
- Link-time optimization (LTO)
- Code size optimization
- Aggressive inlining
- Dead code elimination

Expected output size: ~150-200KB (before gzip)

## Run Demo

### Start Local Server

```bash
# Using Python 3
python3 -m http.server 8080

# Using Node.js
npx http-server -p 8080

# Using Rust
cargo install basic-http-server
basic-http-server -a 0.0.0.0:8080
```

### Open in Browser

Navigate to: http://localhost:8080/www/

## Project Output Structure

After building, you'll have:

```
datagrid5/
├── pkg/
│   ├── datagrid5.js          # JavaScript bindings
│   ├── datagrid5_bg.wasm     # WebAssembly binary
│   ├── datagrid5.d.ts        # TypeScript definitions
│   └── package.json          # NPM package info
├── target/                   # Rust build artifacts
└── www/                      # Demo files (use these)
    ├── index.html
    └── styles.css
```

## Testing

### Run Rust Tests

```bash
cargo test
```

### Run WASM Tests

```bash
wasm-pack test --headless --firefox
wasm-pack test --headless --chrome
```

## Troubleshooting

### Error: "wasm-pack not found"

Install wasm-pack using the methods above, then restart your terminal.

### Error: "Failed to fetch"

Make sure you're serving the files over HTTP, not opening them directly in the browser (file://).
WebAssembly modules must be served with proper MIME types.

### Error: "WebGL not supported"

This library requires WebGL support. Check if your browser has WebGL enabled:
- Visit: https://get.webgl.org/

### Performance Issues

1. Make sure you're using the **release build** (`--release` flag)
2. Check browser console for errors
3. Reduce grid size (rows/columns) for testing
4. Try a different browser (Chrome/Firefox recommended)

## Browser Compatibility

Tested on:
- Chrome/Edge 90+
- Firefox 88+
- Safari 14+

Requires:
- WebAssembly support
- WebGL 1.0 support
- ES6 modules support

## Optimizations

The release build applies these optimizations:

1. **LTO** (Link-Time Optimization): Enables cross-crate optimizations
2. **opt-level = 3**: Maximum optimization level
3. **codegen-units = 1**: Single codegen unit for better optimization
4. **wee_alloc**: Lightweight allocator (reduces WASM size by ~10KB)

## Development Tips

### Quick Rebuild

```bash
# Watch for changes and rebuild
cargo watch -i pkg -s "wasm-pack build --target web --dev"
```

### Format Code

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

### Check Compilation

```bash
cargo check --target wasm32-unknown-unknown
```

## Deployment

For production deployment:

1. Build with release flag
2. Compress WASM file with gzip
3. Serve with proper MIME types:
   - `.wasm` → `application/wasm`
   - `.js` → `application/javascript`
4. Enable CORS if serving from different domain
5. Add cache headers for WASM files

### Example nginx config

```nginx
location /pkg/ {
    types {
        application/wasm wasm;
    }
    gzip on;
    gzip_types application/wasm;
    add_header Cache-Control "public, max-age=31536000";
}
```

## Performance Benchmarking

To measure performance:

1. Open demo in browser
2. Open DevTools (F12)
3. Check Console for FPS counter
4. Use Performance tab to profile
5. Monitor memory usage in Task Manager

## Next Steps

After successful build:

1. Test the demo at http://localhost:8080/www/
2. Try different grid sizes
3. Check browser console for any errors
4. Review TASK.md for roadmap
5. Contribute improvements!

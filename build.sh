#!/bin/bash

# DataGrid5 Build Script
# Builds WebAssembly package using wasm-pack

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored message
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Parse command line arguments first (before checks)
BUILD_TYPE="release"
CLEAN_BUILD=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --dev)
            BUILD_TYPE="dev"
            shift
            ;;
        --release)
            BUILD_TYPE="release"
            shift
            ;;
        --clean)
            CLEAN_BUILD=true
            shift
            ;;
        -h|--help)
            echo "Usage: ./build.sh [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --dev         Build in development mode (faster, larger)"
            echo "  --release     Build in release mode (slower, optimized) [default]"
            echo "  --clean       Clean build artifacts before building"
            echo "  -h, --help    Show this help message"
            echo ""
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Print header
echo ""
echo "╔════════════════════════════════════════╗"
echo "║   DataGrid5 WebAssembly Build Script  ║"
echo "╔════════════════════════════════════════╗"
echo ""

# Check if wasm-pack is installed
print_info "Checking for wasm-pack..."
if ! command -v wasm-pack &> /dev/null; then
    print_error "wasm-pack is not installed!"
    echo ""
    echo "Please install wasm-pack using one of these methods:"
    echo ""
    echo "  1. Using the installer:"
    echo "     curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    echo ""
    echo "  2. Using cargo:"
    echo "     cargo install wasm-pack"
    echo ""
    exit 1
fi

print_success "wasm-pack found: $(wasm-pack --version)"

# Check if wasm32 target is installed
print_info "Checking for wasm32-unknown-unknown target..."
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    print_warning "wasm32-unknown-unknown target not installed"
    print_info "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
    print_success "Target installed"
else
    print_success "wasm32-unknown-unknown target is installed"
fi

# Clean build if requested
if [ "$CLEAN_BUILD" = true ]; then
    print_info "Cleaning build artifacts..."
    rm -rf target/
    rm -rf pkg/
    print_success "Build artifacts cleaned"
fi

# Build
echo ""
print_info "Building DataGrid5 (${BUILD_TYPE} mode)..."
echo ""

START_TIME=$(date +%s)

if [ "$BUILD_TYPE" = "dev" ]; then
    wasm-pack build --target web --dev
else
    wasm-pack build --target web --release
fi

END_TIME=$(date +%s)
BUILD_TIME=$((END_TIME - START_TIME))

echo ""
print_success "Build completed in ${BUILD_TIME} seconds!"
echo ""

# Check if pkg directory was created
if [ ! -d "pkg" ]; then
    print_error "Build failed: pkg directory not found"
    exit 1
fi

# Display build artifacts
print_info "Build artifacts in pkg/:"
echo ""
ls -lh pkg/ | tail -n +2 | while read -r line; do
    echo "  $line"
done
echo ""

# Get file sizes
if [ -f "pkg/datagrid5_bg.wasm" ]; then
    WASM_SIZE=$(du -h pkg/datagrid5_bg.wasm | cut -f1)
    print_info "WASM file size: ${WASM_SIZE}"
fi

if [ -f "pkg/datagrid5.js" ]; then
    JS_SIZE=$(du -h pkg/datagrid5.js | cut -f1)
    print_info "JS bindings size: ${JS_SIZE}"
fi

echo ""
print_success "Build complete! Ready to run examples."
echo ""
echo "To test the build:"
echo "  1. Start a local server:"
echo "     python3 -m http.server 8080"
echo ""
echo "  2. Open in browser:"
echo "     http://localhost:8080/www/"
echo "     http://localhost:8080/examples/"
echo ""

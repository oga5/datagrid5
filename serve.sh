#!/bin/bash

# DataGrid5 Development Server Script
# Starts a local HTTP server for testing

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Default port
PORT=8080

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -p|--port)
            PORT="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: ./serve.sh [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -p, --port PORT    Port to use (default: 8080)"
            echo "  -h, --help         Show this help message"
            echo ""
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Check if pkg directory exists
if [ ! -d "pkg" ]; then
    print_warning "pkg directory not found. Please build first:"
    echo ""
    echo "  ./build.sh"
    echo ""
    exit 1
fi

echo ""
echo "╔════════════════════════════════════════╗"
echo "║   DataGrid5 Development Server         ║"
echo "╔════════════════════════════════════════╗"
echo ""

print_info "Starting HTTP server on port ${PORT}..."
echo ""

# Try different server options
if command -v python3 &> /dev/null; then
    print_success "Using Python 3"
    echo ""
    echo "═══════════════════════════════════════"
    echo "  Server running at:"
    echo "  http://localhost:${PORT}"
    echo ""
    echo "  Examples:"
    echo "  • http://localhost:${PORT}/www/"
    echo "  • http://localhost:${PORT}/examples/"
    echo "  • http://localhost:${PORT}/examples/full-screen-resize-example.html"
    echo "  • http://localhost:${PORT}/examples/responsive-resize-example.html"
    echo ""
    echo "  Press Ctrl+C to stop the server"
    echo "═══════════════════════════════════════"
    echo ""
    python3 -m http.server $PORT
elif command -v python &> /dev/null; then
    print_success "Using Python 2"
    echo ""
    echo "═══════════════════════════════════════"
    echo "  Server running at:"
    echo "  http://localhost:${PORT}"
    echo ""
    echo "  Press Ctrl+C to stop the server"
    echo "═══════════════════════════════════════"
    echo ""
    python -m SimpleHTTPServer $PORT
elif command -v php &> /dev/null; then
    print_success "Using PHP"
    echo ""
    echo "═══════════════════════════════════════"
    echo "  Server running at:"
    echo "  http://localhost:${PORT}"
    echo ""
    echo "  Press Ctrl+C to stop the server"
    echo "═══════════════════════════════════════"
    echo ""
    php -S localhost:$PORT
else
    print_warning "No suitable HTTP server found!"
    echo ""
    echo "Please install one of the following:"
    echo "  • Python 3: apt-get install python3"
    echo "  • Python 2: apt-get install python"
    echo "  • PHP: apt-get install php"
    echo ""
    echo "Or use npx:"
    echo "  npx http-server -p ${PORT}"
    echo ""
    exit 1
fi

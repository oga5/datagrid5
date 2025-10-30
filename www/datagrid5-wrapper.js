/**
 * DataGrid5 JavaScript Wrapper
 * Provides high-level API for common grid operations
 */

export class DataGridWrapper {
    constructor(containerId, wasmModule, options = {}) {
        this.containerId = containerId;
        this.DataGrid = wasmModule.DataGrid;
        this.options = {
            rows: options.rows || 100,
            cols: options.cols || 26,
            enableEditing: options.enableEditing !== false,
            enableVirtualScroll: options.enableVirtualScroll !== false,
            enableResize: options.enableResize !== false,
            ...options
        };

        this.grid = null;
        this.container = null;
        this.webglCanvas = null;
        this.textCanvas = null;
        this.scrollContainer = null;
        this.scrollContent = null;
        this.cellEditor = null;
        this.isInternalScroll = false;
        this.editingRow = null;
        this.editingCol = null;
        this.renderLoopId = null;
        this.clipboardData = ''; // Fallback clipboard storage
        this.isDirty = false; // Track if render is needed
        this.renderScheduled = false; // Track if render is already scheduled

        this.init();
    }

    init() {
        this.container = document.getElementById(this.containerId);
        if (!this.container) {
            throw new Error(`Container '${this.containerId}' not found`);
        }

        this.setupDOM();
        this.createGrid();
        this.setupEventHandlers();

        if (this.options.enableVirtualScroll) {
            this.setupVirtualScroll();
        }

        if (this.options.enableEditing) {
            this.setupCellEditor();
        }

        // Initial render is not needed - user should call render() after loading data
        // this.requestRender();
    }

    setupDOM() {
        // Clear container
        this.container.innerHTML = '';
        this.container.style.position = 'relative';
        this.container.style.width = '100%';
        this.container.style.height = '100%';
        this.container.style.overflow = 'hidden';

        // Create scroll container
        this.scrollContainer = document.createElement('div');
        this.scrollContainer.id = 'scroll-container';
        this.scrollContainer.style.position = 'absolute';
        this.scrollContainer.style.top = '0';
        this.scrollContainer.style.left = '0';
        this.scrollContainer.style.right = '0';
        this.scrollContainer.style.bottom = '0';
        this.scrollContainer.style.overflow = 'auto';
        this.scrollContainer.style.zIndex = '3';

        this.scrollContent = document.createElement('div');
        this.scrollContent.id = 'scroll-content';
        this.scrollContent.style.position = 'relative';
        this.scrollContent.style.pointerEvents = 'none';

        this.scrollContainer.appendChild(this.scrollContent);
        this.container.appendChild(this.scrollContainer);

        // Get container size
        const rect = this.scrollContainer.getBoundingClientRect();
        const width = Math.floor(rect.width);
        const height = Math.floor(rect.height);

        // Create WebGL canvas
        this.webglCanvas = document.createElement('canvas');
        this.webglCanvas.id = 'webgl-canvas';
        this.webglCanvas.width = width;
        this.webglCanvas.height = height;
        this.webglCanvas.style.position = 'absolute';
        this.webglCanvas.style.top = '0';
        this.webglCanvas.style.left = '0';
        this.webglCanvas.style.zIndex = '1';
        this.webglCanvas.style.pointerEvents = 'none';
        this.container.appendChild(this.webglCanvas);

        // Create text canvas
        this.textCanvas = document.createElement('canvas');
        this.textCanvas.id = 'text-canvas';
        this.textCanvas.width = width;
        this.textCanvas.height = height;
        this.textCanvas.tabIndex = 0;
        this.textCanvas.style.position = 'absolute';
        this.textCanvas.style.top = '0';
        this.textCanvas.style.left = '0';
        this.textCanvas.style.zIndex = '2';
        this.textCanvas.style.pointerEvents = 'all';
        this.textCanvas.style.cursor = 'cell';
        this.container.appendChild(this.textCanvas);

        // Setup resize observer
        const resizeObserver = new ResizeObserver(entries => {
            for (let entry of entries) {
                if (!this.grid) return;

                const width = Math.floor(entry.contentRect.width);
                const height = Math.floor(entry.contentRect.height);

                // Only resize if dimensions actually changed
                if (this.webglCanvas.width === width && this.webglCanvas.height === height) {
                    return;
                }

                this.webglCanvas.width = width;
                this.webglCanvas.height = height;
                this.textCanvas.width = width;
                this.textCanvas.height = height;

                this.grid.resize(width, height);
                this.updateVirtualScrollSize();
                this.requestRender();
            }
        });

        resizeObserver.observe(this.scrollContainer);
    }

    createGrid() {
        if (!this.DataGrid) {
            throw new Error('DataGrid class is not defined. Make sure to pass { DataGrid } to DataGridWrapper constructor.');
        }
        this.grid = new this.DataGrid(
            'webgl-canvas',
            'text-canvas',
            this.options.rows,
            this.options.cols
        );
    }

    setupEventHandlers() {
        // Mouse events
        this.textCanvas.addEventListener('mousedown', (e) => {
            const rect = this.textCanvas.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;

            // Check for resize handle
            const resizeType = this.grid.check_resize_handle(x, y);
            if (resizeType !== 'none') {
                this.grid.start_resize(x, y, resizeType);
                e.preventDefault();
                return;
            }

            // Handle cell selection with modifiers
            this.grid.handle_mouse_down_at_with_modifiers(
                x, y,
                e.shiftKey,
                e.ctrlKey || e.metaKey
            );
            this.requestRender();
        });

        this.textCanvas.addEventListener('mousemove', (e) => {
            const rect = this.textCanvas.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;

            // Update cursor for resize handles
            const resizeType = this.grid.check_resize_handle(x, y);
            if (resizeType === 'col') {
                this.textCanvas.style.cursor = 'col-resize';
            } else if (resizeType === 'row') {
                this.textCanvas.style.cursor = 'row-resize';
            } else {
                this.textCanvas.style.cursor = 'cell';
            }

            // Handle mouse move
            if (this.grid.is_resizing()) {
                this.grid.update_resize(x, y);
                this.requestRender();
            } else {
                this.grid.handle_mouse_move(e);
                // Don't render on every mouse move unless needed
            }
        });

        this.textCanvas.addEventListener('mouseup', (e) => {
            const rect = this.textCanvas.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;

            if (this.grid.is_resizing()) {
                this.grid.end_resize();
                this.requestRender();
            } else {
                this.grid.handle_mouse_up(x, y);
                this.requestRender();
            }
        });

        // Double-click for editing
        if (this.options.enableEditing) {
            this.textCanvas.addEventListener('dblclick', (e) => {
                const rect = this.textCanvas.getBoundingClientRect();
                const x = e.clientX - rect.left;
                const y = e.clientY - rect.top;

                const cellInfo = this.grid.handle_double_click(x, y);
                if (cellInfo) {
                    const [row, col] = JSON.parse(cellInfo);
                    this.startCellEdit(row, col);
                }
            });
        }

        // Keyboard events
        this.textCanvas.addEventListener('keydown', (e) => {
            const isCtrl = e.ctrlKey || e.metaKey;

            // Handle clipboard operations
            if (isCtrl) {
                if (e.key === 'c' || e.key === 'C') {
                    // Copy
                    e.preventDefault();
                    this.handleCopy();
                    return;
                } else if (e.key === 'x' || e.key === 'X') {
                    // Cut
                    e.preventDefault();
                    this.handleCut();
                    return;
                } else if (e.key === 'v' || e.key === 'V') {
                    // Paste
                    e.preventDefault();
                    this.handlePaste();
                    return;
                }
            }

            const handled = this.grid.handle_keyboard_with_modifiers(
                e.key,
                isCtrl,
                e.shiftKey
            );

            if (handled) {
                e.preventDefault();
                this.syncScrollPosition();
                this.requestRender();
            }
        });

        // Wheel events
        this.textCanvas.addEventListener('wheel', (e) => {
            e.preventDefault();
            this.grid.handle_wheel(e.deltaX, e.deltaY);
            this.requestRender();
        });

        // Context menu
        this.textCanvas.addEventListener('contextmenu', (e) => {
            e.preventDefault();
            const contextInfo = this.grid.handle_context_menu(e);
            if (contextInfo) {
                const info = JSON.parse(contextInfo);
                // Emit custom event for application to handle
                this.container.dispatchEvent(new CustomEvent('gridcontextmenu', {
                    detail: info
                }));
            }
        });
    }

    setupVirtualScroll() {
        this.updateVirtualScrollSize();

        this.scrollContainer.addEventListener('scroll', () => {
            if (!this.grid || this.isInternalScroll) {
                this.isInternalScroll = false;
                return;
            }

            const scrollX = this.scrollContainer.scrollLeft;
            const scrollY = this.scrollContainer.scrollTop;
            this.grid.set_scroll(scrollX, scrollY);
            this.requestRender();
        });
    }

    updateVirtualScrollSize() {
        if (!this.grid) return;

        const totalSize = JSON.parse(this.grid.get_total_size());
        const [totalWidth, totalHeight] = totalSize;

        this.scrollContent.style.width = totalWidth + 'px';
        this.scrollContent.style.height = totalHeight + 'px';
    }

    syncScrollPosition() {
        if (!this.grid) return;

        const viewport = JSON.parse(this.grid.get_viewport_info_array());
        const [canvasWidth, canvasHeight, scrollY, scrollX] = viewport;

        this.isInternalScroll = true;
        this.scrollContainer.scrollLeft = scrollX;
        this.scrollContainer.scrollTop = scrollY;
    }

    setupCellEditor() {
        this.cellEditor = document.createElement('input');
        this.cellEditor.type = 'text';
        this.cellEditor.id = 'cell-editor';
        this.cellEditor.style.position = 'absolute';
        this.cellEditor.style.border = '2px solid #667eea';
        this.cellEditor.style.outline = 'none';
        this.cellEditor.style.fontFamily = 'Arial, sans-serif';
        this.cellEditor.style.fontSize = '14px';
        this.cellEditor.style.padding = '4px';
        this.cellEditor.style.boxSizing = 'border-box';
        this.cellEditor.style.zIndex = '1000';
        this.cellEditor.style.background = 'white';
        this.cellEditor.style.display = 'none';
        this.container.appendChild(this.cellEditor);
    }

    startCellEdit(row, col) {
        if (!this.options.enableEditing || !this.cellEditor) return;

        // Start edit in grid
        const canEdit = this.grid.start_edit(row, col);
        if (!canEdit) {
            console.log('Cannot edit this cell');
            return;
        }

        // Get cell position
        const rect = this.grid.get_cell_edit_rect(row, col);
        const [x, y, width, height] = rect;

        // Get current value
        const currentValue = this.grid.get_cell_value(row, col);

        // Calculate position
        const scrollContainerRect = this.scrollContainer.getBoundingClientRect();
        const containerRect = this.container.getBoundingClientRect();
        const offsetX = scrollContainerRect.left - containerRect.left;
        const offsetY = scrollContainerRect.top - containerRect.top;

        // Position editor
        this.cellEditor.style.left = `${x + offsetX - this.scrollContainer.scrollLeft}px`;
        this.cellEditor.style.top = `${y + offsetY - this.scrollContainer.scrollTop}px`;
        this.cellEditor.style.width = `${width}px`;
        this.cellEditor.style.height = `${height}px`;
        this.cellEditor.style.display = 'block';
        this.cellEditor.value = currentValue;

        this.editingRow = row;
        this.editingCol = col;

        this.cellEditor.focus();
        this.cellEditor.select();

        this.setupEditorEvents();

        // Emit edit start event
        this.container.dispatchEvent(new CustomEvent('celleditstart', {
            detail: { row, col, value: currentValue }
        }));
    }

    endCellEdit(save = false, moveDown = false, moveRight = false) {
        if (!this.grid || this.editingRow === null || this.editingCol === null) return;

        const oldValue = this.grid.get_cell_value(this.editingRow, this.editingCol);
        let changed = false;

        if (save && this.cellEditor.value !== oldValue) {
            // Save value
            this.grid.update_cell_value(this.editingRow, this.editingCol, this.cellEditor.value);
            changed = true;
            this.requestRender();
        }

        // Hide editor
        this.cellEditor.style.display = 'none';
        this.cellEditor.value = '';

        // End edit mode
        this.grid.end_edit();

        const currentRow = this.editingRow;
        const currentCol = this.editingCol;

        this.editingRow = null;
        this.editingCol = null;

        // Emit edit end event
        this.container.dispatchEvent(new CustomEvent('celleditend', {
            detail: {
                row: currentRow,
                col: currentCol,
                oldValue,
                newValue: save ? this.cellEditor.value : oldValue,
                changed,
                saved: save
            }
        }));

        // Move to next cell if requested
        const maxRow = this.grid.row_count() - 1;
        const maxCol = this.grid.col_count() - 1;

        if (save && moveDown && currentRow < maxRow) {
            setTimeout(() => this.startCellEdit(currentRow + 1, currentCol), 10);
        } else if (save && moveRight && currentCol < maxCol) {
            setTimeout(() => this.startCellEdit(currentRow, currentCol + 1), 10);
        } else {
            this.textCanvas.focus();
        }
    }

    setupEditorEvents() {
        // Remove old listeners by cloning
        const newEditor = this.cellEditor.cloneNode(true);
        this.cellEditor.parentNode.replaceChild(newEditor, this.cellEditor);
        this.cellEditor = newEditor;

        // Enter to save and move down
        newEditor.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                e.preventDefault();
                this.endCellEdit(true, true, false);
            } else if (e.key === 'Tab') {
                e.preventDefault();
                this.endCellEdit(true, false, true);
            } else if (e.key === 'Escape') {
                e.preventDefault();
                this.endCellEdit(false);
            }
        });

        // Click outside to save
        newEditor.addEventListener('blur', () => {
            setTimeout(() => this.endCellEdit(true), 100);
        });
    }

    // Request a render on the next animation frame (event-driven rendering)
    requestRender() {
        if (this.renderScheduled) return;

        this.renderScheduled = true;
        requestAnimationFrame(() => {
            this.renderScheduled = false;
            if (this.grid) {
                this.grid.render();
            }
        });
    }

    destroy() {
        if (this.container) {
            this.container.innerHTML = '';
        }
        this.grid = null;
    }

    // Expose common grid methods
    getGrid() {
        return this.grid;
    }

    // Public method to request a render (async via requestAnimationFrame)
    render() {
        this.requestRender();
    }

    // Public method to force immediate synchronous render
    renderNow() {
        if (this.grid) {
            this.grid.render();
        }
    }

    getCellValue(row, col) {
        return this.grid.get_cell_value(row, col);
    }

    setCellValue(row, col, value) {
        this.grid.set_cell_value(row, col, value);
        this.requestRender();
    }

    getSelectedCell() {
        const selected = this.grid.get_selected_cell();
        return selected ? JSON.parse(selected) : null;
    }

    getSelectedCells() {
        const selected = this.grid.get_selected_cells();
        return selected ? JSON.parse(selected) : [];
    }

    setScroll(x, y) {
        this.grid.set_scroll(x, y);
        this.requestRender();
    }

    getViewportInfo() {
        const info = this.grid.get_viewport_info_array();
        return JSON.parse(info);
    }

    // Clipboard operations
    handleCopy() {
        try {
            const tsvData = this.grid.copy_selected_cells();

            if (tsvData) {
                // Copy to system clipboard
                navigator.clipboard.writeText(tsvData).then(() => {
                    console.log('Copied to clipboard');

                    // Emit custom event
                    this.container.dispatchEvent(new CustomEvent('gridcopy', {
                        detail: { data: tsvData }
                    }));
                }).catch(err => {
                    console.error('Failed to copy to clipboard:', err);

                    // Fallback: store in memory
                    this.clipboardData = tsvData;
                    this.container.dispatchEvent(new CustomEvent('gridcopy', {
                        detail: { data: tsvData, fallback: true }
                    }));
                });
            }
        } catch (err) {
            console.error('Copy error:', err);
        }
    }

    handleCut() {
        try {
            const tsvData = this.grid.cut_selected_cells();

            if (tsvData) {
                this.requestRender();

                // Copy to system clipboard
                navigator.clipboard.writeText(tsvData).then(() => {
                    console.log('Cut to clipboard');

                    // Emit custom event
                    this.container.dispatchEvent(new CustomEvent('gridcut', {
                        detail: { data: tsvData }
                    }));
                }).catch(err => {
                    console.error('Failed to cut to clipboard:', err);

                    // Fallback: store in memory
                    this.clipboardData = tsvData;
                    this.container.dispatchEvent(new CustomEvent('gridcut', {
                        detail: { data: tsvData, fallback: true }
                    }));
                });
            }
        } catch (err) {
            console.error('Cut error:', err);
        }
    }

    async handlePaste() {
        try {
            let tsvData;

            // Try to read from system clipboard
            try {
                tsvData = await navigator.clipboard.readText();
            } catch (err) {
                console.log('Cannot read from clipboard, using fallback');
                // Fallback: use memory clipboard
                tsvData = this.clipboardData || '';
            }

            if (tsvData) {
                // Paste into grid
                this.grid.paste_cells(tsvData);
                console.log('Pasted from clipboard');

                // Emit custom event
                this.container.dispatchEvent(new CustomEvent('gridpaste', {
                    detail: { data: tsvData }
                }));

                this.requestRender();
            }
        } catch (err) {
            console.error('Paste error:', err);
        }
    }

    // Manual clipboard methods
    copy() {
        this.handleCopy();
    }

    cut() {
        this.handleCut();
    }

    paste(tsvData) {
        if (tsvData) {
            this.grid.paste_cells(tsvData);
            this.requestRender();
        } else {
            this.handlePaste();
        }
    }
}

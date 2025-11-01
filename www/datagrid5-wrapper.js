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
            blurBehavior: options.blurBehavior || 'save', // 'save' or 'cancel'
            saveOnScroll: options.saveOnScroll !== false, // Save on scroll (default: true)
            debug: options.debug || false, // Enable debug logging (default: false)
            ...options
        };

        // DOM references
        this.grid = null;
        this.container = null;
        this.webglCanvas = null;
        this.textCanvas = null;
        this.scrollContainer = null;
        this.scrollContent = null;
        this.cellEditor = null;

        // State
        this.isInternalScroll = false;
        this.editingRow = null;
        this.editingCol = null;
        this.renderLoopId = null;
        this.clipboardData = ''; // Fallback clipboard storage
        this.isDirty = false; // Track if render is needed
        this.renderScheduled = false; // Track if render is already scheduled
        this.isComposing = false; // Track IME composition state
        this.scrollScheduled = false; // Track if scroll render is scheduled

        // Observers and handlers (for proper cleanup)
        this.resizeObserver = null;
        this.documentClickHandler = null;

        // Bind event handlers to this instance for proper cleanup
        this._onTextCanvasMouseDown = this._onTextCanvasMouseDown.bind(this);
        this._onTextCanvasMouseMove = this._onTextCanvasMouseMove.bind(this);
        this._onTextCanvasMouseUp = this._onTextCanvasMouseUp.bind(this);
        this._onTextCanvasDoubleClick = this._onTextCanvasDoubleClick.bind(this);
        this._onTextCanvasKeyDown = this._onTextCanvasKeyDown.bind(this);
        this._onTextCanvasWheel = this._onTextCanvasWheel.bind(this);
        this._onTextCanvasContextMenu = this._onTextCanvasContextMenu.bind(this);
        this._onTextCanvasFocus = this._onTextCanvasFocus.bind(this);
        this._onTextCanvasBlur = this._onTextCanvasBlur.bind(this);
        this._onScroll = this._onScroll.bind(this);
        this._onEditorCompositionStart = this._onEditorCompositionStart.bind(this);
        this._onEditorCompositionUpdate = this._onEditorCompositionUpdate.bind(this);
        this._onEditorCompositionEnd = this._onEditorCompositionEnd.bind(this);
        this._onEditorKeyDown = this._onEditorKeyDown.bind(this);
        this._onEditorFocus = this._onEditorFocus.bind(this);
        this._onEditorBlur = this._onEditorBlur.bind(this);

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

    // Debug logging helper
    _log(...args) {
        if (this.options.debug) {
            console.log('[Wrapper]', ...args);
        }
    }

    setupDOM() {
        // Clear container
        this.container.innerHTML = '';
        this.container.style.position = 'relative';
        this.container.style.width = '100%';
        this.container.style.height = '100%';
        this.container.style.overflow = 'hidden';

        // ARIA attributes for accessibility
        this.container.setAttribute('role', 'application');
        this.container.setAttribute('aria-label', 'Data Grid');

        // Create scroll container
        this.scrollContainer = document.createElement('div');
        this.scrollContainer.id = 'scroll-container';
        this.scrollContainer.style.position = 'absolute';
        this.scrollContainer.style.top = '0';
        this.scrollContainer.style.left = '0';
        this.scrollContainer.style.right = '0';
        this.scrollContainer.style.bottom = '0';
        this.scrollContainer.style.overflow = 'auto';
        this.scrollContainer.style.zIndex = '1';

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

        // Get device pixel ratio for high-DPI displays
        const dpr = window.devicePixelRatio || 1;

        // Create WebGL canvas
        this.webglCanvas = document.createElement('canvas');
        this.webglCanvas.id = 'webgl-canvas';
        // Set actual canvas size (CSS size * device pixel ratio)
        this.webglCanvas.width = width * dpr;
        this.webglCanvas.height = height * dpr;
        // Set display size (CSS pixels)
        this.webglCanvas.style.width = width + 'px';
        this.webglCanvas.style.height = height + 'px';
        this.webglCanvas.style.position = 'absolute';
        this.webglCanvas.style.top = '0';
        this.webglCanvas.style.left = '0';
        this.webglCanvas.style.zIndex = '2';
        this.webglCanvas.style.pointerEvents = 'none';
        this.webglCanvas.setAttribute('aria-hidden', 'true');
        this.container.appendChild(this.webglCanvas);

        // Create text canvas
        this.textCanvas = document.createElement('canvas');
        this.textCanvas.id = 'text-canvas';
        // Set actual canvas size (CSS size * device pixel ratio)
        this.textCanvas.width = width * dpr;
        this.textCanvas.height = height * dpr;
        // Set display size (CSS pixels)
        this.textCanvas.style.width = width + 'px';
        this.textCanvas.style.height = height + 'px';
        this.textCanvas.tabIndex = 0;
        this.textCanvas.style.position = 'absolute';
        this.textCanvas.style.top = '0';
        this.textCanvas.style.left = '0';
        this.textCanvas.style.zIndex = '3';
        this.textCanvas.style.pointerEvents = 'all';
        this.textCanvas.style.cursor = 'cell';

        // ARIA attributes for grid interaction
        this.textCanvas.setAttribute('role', 'grid');
        this.textCanvas.setAttribute('aria-label', `Data grid with ${this.options.rows} rows and ${this.options.cols} columns`);
        this.textCanvas.setAttribute('aria-readonly', this.options.enableEditing ? 'false' : 'true');

        // High-contrast focus indicator for accessibility
        this.textCanvas.style.outline = 'none';
        this.textCanvas.addEventListener('focus', this._onTextCanvasFocus);
        this.textCanvas.addEventListener('blur', this._onTextCanvasBlur);

        this.container.appendChild(this.textCanvas);

        // Setup resize observer (store reference for cleanup)
        this.resizeObserver = new ResizeObserver(entries => {
            for (let entry of entries) {
                if (!this.grid) return;

                const width = Math.floor(entry.contentRect.width);
                const height = Math.floor(entry.contentRect.height);
                const dpr = window.devicePixelRatio || 1;

                // Check if dimensions changed (compare CSS size, not canvas buffer size)
                const currentCssWidth = parseInt(this.webglCanvas.style.width);
                const currentCssHeight = parseInt(this.webglCanvas.style.height);

                if (currentCssWidth === width && currentCssHeight === height) {
                    return;
                }

                this._log('Resizing canvas:', width, height, 'DPR:', dpr);

                // Update canvas sizes with device pixel ratio
                this.webglCanvas.width = width * dpr;
                this.webglCanvas.height = height * dpr;
                this.webglCanvas.style.width = width + 'px';
                this.webglCanvas.style.height = height + 'px';

                this.textCanvas.width = width * dpr;
                this.textCanvas.height = height * dpr;
                this.textCanvas.style.width = width + 'px';
                this.textCanvas.style.height = height + 'px';

                // Pass CSS size to grid (not buffer size)
                // TODO: Consider passing DPR to grid if it needs buffer size
                this.grid.resize(width, height);
                this.updateVirtualScrollSize();
                this.requestRender();
            }
        });

        this.resizeObserver.observe(this.scrollContainer);
    }

    // Text canvas event handlers
    _onTextCanvasFocus() {
        this.textCanvas.style.outline = '3px solid #667eea';
        this.textCanvas.style.outlineOffset = '-3px';
    }

    _onTextCanvasBlur() {
        this.textCanvas.style.outline = 'none';
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
        // Register all event listeners using bound methods (for proper cleanup)
        this.textCanvas.addEventListener('mousedown', this._onTextCanvasMouseDown);
        this.textCanvas.addEventListener('mousemove', this._onTextCanvasMouseMove);
        this.textCanvas.addEventListener('mouseup', this._onTextCanvasMouseUp);

        if (this.options.enableEditing) {
            this.textCanvas.addEventListener('dblclick', this._onTextCanvasDoubleClick);
        }

        this.textCanvas.addEventListener('keydown', this._onTextCanvasKeyDown);
        this.textCanvas.addEventListener('wheel', this._onTextCanvasWheel);
        this.textCanvas.addEventListener('contextmenu', this._onTextCanvasContextMenu);
    }

    _onTextCanvasMouseDown(e) {
        this._log('mousedown event fired', e.clientX, e.clientY);
        const rect = this.textCanvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        this._log('canvas coords:', x, y);

        // Check for resize handle
        const resizeType = this.grid.check_resize_handle(x, y);
        this._log('resizeType:', resizeType);
        if (resizeType !== 'none') {
            this.grid.start_resize(x, y, resizeType);
            e.preventDefault();
            return;
        }

        // Handle cell selection with modifiers
        this._log('calling handle_mouse_down_at_with_modifiers');
        this.grid.handle_mouse_down_at_with_modifiers(
            x, y,
            e.shiftKey,
            e.ctrlKey || e.metaKey
        );
        this._log('calling requestRender');
        this.requestRender();
    }

    _onTextCanvasMouseMove(e) {
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
            // Render if drag-selecting to show live selection preview
            if (this.grid.is_selecting()) {
                this.requestRender();
            }
        }
    }

    _onTextCanvasMouseUp(e) {
        const rect = this.textCanvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;

        if (this.grid.is_resizing()) {
            this.grid.end_resize();
            this.updateVirtualScrollSize();  // Update scroll size after resize
            this.requestRender();
        } else {
            this.grid.handle_mouse_up(x, y);
            this.requestRender();
        }
    }

    _onTextCanvasDoubleClick(e) {
        const rect = this.textCanvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;

        const cellInfo = this.grid.handle_double_click_at(x, y);
        if (cellInfo) {
            const [row, col] = JSON.parse(cellInfo);
            this.startCellEdit(row, col);
        }
    }

    _onTextCanvasKeyDown(e) {
        // Check if key is defined
        if (!e.key) {
            return;
        }

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

        const handled = this.grid.handle_keyboard_with_modifiers_key(
            e.key,
            isCtrl,
            e.shiftKey
        );

        if (handled) {
            e.preventDefault();
            this.syncScrollPosition();
            this.requestRender();
        }
    }

    _onTextCanvasWheel(e) {
        e.preventDefault();
        this.grid.handle_wheel(e.deltaX, e.deltaY);
        this.syncScrollPosition();
        this.requestRender();
    }

    _onTextCanvasContextMenu(e) {
        e.preventDefault();
        const contextInfo = this.grid.handle_context_menu(e);
        if (contextInfo) {
            const info = JSON.parse(contextInfo);
            // Emit custom event for application to handle
            this.container.dispatchEvent(new CustomEvent('gridcontextmenu', {
                detail: info
            }));
        }
    }

    setupVirtualScroll() {
        this.updateVirtualScrollSize();
        // Register scroll event listener with bound method
        this.scrollContainer.addEventListener('scroll', this._onScroll, { passive: true });
    }

    _onScroll() {
        if (!this.grid || this.isInternalScroll) {
            this.isInternalScroll = false;
            return;
        }

        // Use rAF-based throttling to prevent excessive scroll processing
        if (this.scrollScheduled) {
            return;
        }

        this.scrollScheduled = true;
        requestAnimationFrame(() => {
            this.scrollScheduled = false;

            if (!this.grid) return;

            // Handle editing during scroll based on configuration
            if (this.editingRow !== null && this.editingCol !== null) {
                if (this.options.saveOnScroll) {
                    this._log('Scroll detected during edit - saving');
                    // Save and end edit on scroll
                    this.endCellEdit(true);
                } else {
                    // Just update editor position without ending edit
                    // This allows continuous editing while scrolling
                    this.updateEditorPosition();
                }
            }

            const scrollX = this.scrollContainer.scrollLeft;
            const scrollY = this.scrollContainer.scrollTop;
            this.grid.set_scroll(scrollX, scrollY);
            this.requestRender();
        });
    }

    updateEditorPosition() {
        // Update editor position if currently editing
        if (!this.cellEditor || this.editingRow === null || this.editingCol === null) {
            return;
        }

        const rect = this.grid.get_cell_edit_rect(this.editingRow, this.editingCol);
        const [x, y, width, height] = rect;

        const scrollContainerRect = this.scrollContainer.getBoundingClientRect();
        const containerRect = this.container.getBoundingClientRect();
        const offsetX = scrollContainerRect.left - containerRect.left;
        const offsetY = scrollContainerRect.top - containerRect.top;

        this.cellEditor.style.left = `${x + offsetX - this.scrollContainer.scrollLeft}px`;
        this.cellEditor.style.top = `${y + offsetY - this.scrollContainer.scrollTop}px`;
        this.cellEditor.style.width = `${width}px`;
        this.cellEditor.style.height = `${height}px`;
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

        // Prevent browser autocomplete/autocorrect/spellcheck interference
        this.cellEditor.setAttribute('autocomplete', 'off');
        this.cellEditor.setAttribute('autocorrect', 'off');
        this.cellEditor.setAttribute('spellcheck', 'false');

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

        // Setup event listeners once
        this.setupEditorEvents();
    }

    startCellEdit(row, col) {
        if (!this.options.enableEditing || !this.cellEditor) return;

        this._log(`startCellEdit called for (${row}, ${col})`);
        this._log(`Current editing state: row=${this.editingRow}, col=${this.editingCol}`);

        // If already editing, end the current edit first
        if (this.editingRow !== null || this.editingCol !== null) {
            this._log('Ending previous edit first');
            this.endCellEdit(true, false, false);
        }

        // Start edit in grid
        this._log(`Calling grid.start_edit(${row}, ${col})`);
        const canEdit = this.grid.start_edit(row, col);
        this._log(`grid.start_edit returned: ${canEdit}`);
        if (!canEdit) {
            this._log('Cannot edit this cell');
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

        // Focus and select after a short delay to ensure DOM is ready
        setTimeout(() => {
            this.cellEditor.focus();
            this.cellEditor.select();
        }, 0);

        // Emit edit start event
        this.container.dispatchEvent(new CustomEvent('celleditstart', {
            detail: { row, col, value: currentValue }
        }));
    }

    endCellEdit(save = false, moveDown = false, moveRight = false, moveLeft = false) {
        if (!this.grid || this.editingRow === null || this.editingCol === null) return;

        this._log(`endCellEdit called: save=${save}, moveDown=${moveDown}, moveRight=${moveRight}, moveLeft=${moveLeft}`);

        const oldValue = this.grid.get_cell_value(this.editingRow, this.editingCol);
        let changed = false;

        if (save && this.cellEditor.value !== oldValue) {
            // Save value
            this.grid.update_cell_value(this.editingRow, this.editingCol, this.cellEditor.value);
            changed = true;
            this.requestRender();
        }

        // Clear editing state immediately to prevent re-entry
        const currentRow = this.editingRow;
        const currentCol = this.editingCol;

        this.editingRow = null;
        this.editingCol = null;

        // Hide editor
        this.cellEditor.style.display = 'none';
        this.cellEditor.value = '';

        // End edit mode
        this.grid.end_edit();

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
            const nextRow = currentRow + 1;
            this.grid.select_cell(nextRow, currentCol);
            this.requestRender();
            setTimeout(() => this.startCellEdit(nextRow, currentCol), 10);
        } else if (save && moveRight && currentCol < maxCol) {
            const nextCol = currentCol + 1;
            this.grid.select_cell(currentRow, nextCol);
            this.requestRender();
            setTimeout(() => this.startCellEdit(currentRow, nextCol), 10);
        } else if (save && moveLeft && currentCol > 0) {
            const nextCol = currentCol - 1;
            this.grid.select_cell(currentRow, nextCol);
            this.requestRender();
            setTimeout(() => this.startCellEdit(currentRow, nextCol), 10);
        } else {
            this.textCanvas.focus();
        }
    }

    setupEditorEvents() {
        // Setup event listeners using bound methods (for proper cleanup)
        this.cellEditor.addEventListener('compositionstart', this._onEditorCompositionStart);
        this.cellEditor.addEventListener('compositionupdate', this._onEditorCompositionUpdate);
        this.cellEditor.addEventListener('compositionend', this._onEditorCompositionEnd);
        this.cellEditor.addEventListener('keydown', this._onEditorKeyDown);
        this.cellEditor.addEventListener('focus', this._onEditorFocus);
        this.cellEditor.addEventListener('blur', this._onEditorBlur);
    }

    _onEditorCompositionStart() {
        this._log('IME composition started');
        this.isComposing = true;
    }

    _onEditorCompositionUpdate(e) {
        this._log('IME composition updating:', e.data);
        // Keep isComposing flag true during updates
        this.isComposing = true;
    }

    _onEditorCompositionEnd(e) {
        this._log('IME composition ended:', e.data);
        // Set flag to false, but with a small delay to ensure
        // the final text is properly inserted before any keydown handler fires
        // This prevents race condition between compositionend and keydown events
        setTimeout(() => {
            this.isComposing = false;
        }, 0);
    }

    _onEditorKeyDown(e) {
        // Don't handle navigation keys during IME composition
        if (this.isComposing) {
            this._log('Ignoring keydown during IME composition:', e.key);
            return;
        }

        if (e.key === 'Enter') {
            e.preventDefault();
            this.endCellEdit(true, true, false);
        } else if (e.key === 'Tab') {
            e.preventDefault();
            if (e.shiftKey) {
                // Shift+Tab: move left
                this.endCellEdit(true, false, false, true);
            } else {
                // Tab: move right
                this.endCellEdit(true, false, true, false);
            }
        } else if (e.key === 'Escape') {
            e.preventDefault();
            this.endCellEdit(false);
        }
    }

    _onEditorFocus() {
        this._log('Editor focused - adding document click listener');
        // Setup document click handler for detecting external clicks
        if (!this.documentClickHandler) {
            this.documentClickHandler = (e) => {
                // Only process if we're currently editing
                if (this.editingRow === null || this.editingCol === null) {
                    return;
                }

                // Check if click is outside the editor and grid
                const clickedElement = e.target;
                const isEditorClick = this.cellEditor.contains(clickedElement);
                const isGridClick = this.container.contains(clickedElement);

                this._log('Document click:', {
                    isEditorClick,
                    isGridClick,
                    target: clickedElement.tagName
                });

                // If clicked outside both editor and grid, end edit
                if (!isEditorClick && !isGridClick) {
                    this._log('External click detected - ending edit');
                    const shouldSave = this.options.blurBehavior === 'save';
                    // Use setTimeout to avoid conflicts with other click handlers
                    setTimeout(() => {
                        if (this.editingRow !== null || this.editingCol !== null) {
                            this.endCellEdit(shouldSave);
                        }
                    }, 0);
                }
            };
        }
        // Add document click listener when editor gains focus
        document.addEventListener('click', this.documentClickHandler, true);
    }

    _onEditorBlur(e) {
        this._log('Editor blur event');
        // Remove document click listener when editor loses focus
        if (this.documentClickHandler) {
            document.removeEventListener('click', this.documentClickHandler, true);
        }

        // Handle blur if we're still editing (edge case: might have been ended already)
        // This handles cases like pressing Tab to move to next cell
        if (this.editingRow !== null && this.editingCol !== null) {
            // If blur was caused by clicking inside the grid, the grid click handler
            // will take care of ending the edit. Otherwise, end it here.
            const relatedTarget = e.relatedTarget;
            if (!relatedTarget || !this.container.contains(relatedTarget)) {
                this._log('Blur without related target in grid - ending edit');
                setTimeout(() => {
                    if (this.editingRow !== null || this.editingCol !== null) {
                        const shouldSave = this.options.blurBehavior === 'save';
                        this.endCellEdit(shouldSave);
                    }
                }, 100);
            }
        }
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
        this._log('Destroying DataGridWrapper');

        // Clean up document click listener if it exists
        if (this.documentClickHandler) {
            document.removeEventListener('click', this.documentClickHandler, true);
            this.documentClickHandler = null;
        }

        // Clean up ResizeObserver
        if (this.resizeObserver) {
            this.resizeObserver.disconnect();
            this.resizeObserver = null;
        }

        // Clean up text canvas event listeners
        if (this.textCanvas) {
            this.textCanvas.removeEventListener('mousedown', this._onTextCanvasMouseDown);
            this.textCanvas.removeEventListener('mousemove', this._onTextCanvasMouseMove);
            this.textCanvas.removeEventListener('mouseup', this._onTextCanvasMouseUp);
            this.textCanvas.removeEventListener('dblclick', this._onTextCanvasDoubleClick);
            this.textCanvas.removeEventListener('keydown', this._onTextCanvasKeyDown);
            this.textCanvas.removeEventListener('wheel', this._onTextCanvasWheel);
            this.textCanvas.removeEventListener('contextmenu', this._onTextCanvasContextMenu);
            this.textCanvas.removeEventListener('focus', this._onTextCanvasFocus);
            this.textCanvas.removeEventListener('blur', this._onTextCanvasBlur);
        }

        // Clean up scroll container event listeners
        if (this.scrollContainer) {
            this.scrollContainer.removeEventListener('scroll', this._onScroll);
        }

        // Clean up cell editor event listeners
        if (this.cellEditor) {
            this.cellEditor.removeEventListener('compositionstart', this._onEditorCompositionStart);
            this.cellEditor.removeEventListener('compositionupdate', this._onEditorCompositionUpdate);
            this.cellEditor.removeEventListener('compositionend', this._onEditorCompositionEnd);
            this.cellEditor.removeEventListener('keydown', this._onEditorKeyDown);
            this.cellEditor.removeEventListener('focus', this._onEditorFocus);
            this.cellEditor.removeEventListener('blur', this._onEditorBlur);
        }

        // Clean up DOM
        if (this.container) {
            this.container.innerHTML = '';
        }

        // Clean up references
        this.grid = null;
        this.cellEditor = null;
        this.webglCanvas = null;
        this.textCanvas = null;
        this.scrollContainer = null;
        this.scrollContent = null;
        this.container = null;
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
                    this._log('Copied to clipboard');

                    // Emit custom event
                    this.container.dispatchEvent(new CustomEvent('gridcopy', {
                        detail: { data: tsvData }
                    }));
                }).catch(err => {
                    console.error('[Wrapper] Failed to copy to clipboard:', err);

                    // Fallback: store in memory
                    this.clipboardData = tsvData;
                    this.container.dispatchEvent(new CustomEvent('gridcopy', {
                        detail: { data: tsvData, fallback: true }
                    }));
                });
            }
        } catch (err) {
            console.error('[Wrapper] Copy error:', err);
        }
    }

    handleCut() {
        try {
            const tsvData = this.grid.cut_selected_cells();

            if (tsvData) {
                this.requestRender();

                // Copy to system clipboard
                navigator.clipboard.writeText(tsvData).then(() => {
                    this._log('Cut to clipboard');

                    // Emit custom event
                    this.container.dispatchEvent(new CustomEvent('gridcut', {
                        detail: { data: tsvData }
                    }));
                }).catch(err => {
                    console.error('[Wrapper] Failed to cut to clipboard:', err);

                    // Fallback: store in memory
                    this.clipboardData = tsvData;
                    this.container.dispatchEvent(new CustomEvent('gridcut', {
                        detail: { data: tsvData, fallback: true }
                    }));
                });
            }
        } catch (err) {
            console.error('[Wrapper] Cut error:', err);
        }
    }

    async handlePaste() {
        try {
            let tsvData;

            // Try to read from system clipboard
            try {
                tsvData = await navigator.clipboard.readText();
            } catch (err) {
                this._log('Cannot read from clipboard, using fallback');
                // Fallback: use memory clipboard
                tsvData = this.clipboardData || '';
            }

            if (tsvData) {
                // Paste into grid
                this.grid.paste_cells(tsvData);
                this._log('Pasted from clipboard');

                // Emit custom event
                this.container.dispatchEvent(new CustomEvent('gridpaste', {
                    detail: { data: tsvData }
                }));

                this.requestRender();
            }
        } catch (err) {
            console.error('[Wrapper] Paste error:', err);
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

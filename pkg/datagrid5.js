let wasm;

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    }
}

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;

function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

let cachedUint32ArrayMemory0 = null;

function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}
/**
 * Initialize the library
 */
export function init() {
    wasm.init();
}

const DataGridFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_datagrid_free(ptr >>> 0, 1));
/**
 * Main DataGrid control
 */
export class DataGrid {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DataGrid.prototype);
        obj.__wbg_ptr = ptr;
        DataGridFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DataGridFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_datagrid_free(ptr, 0);
    }
    /**
     * Create a new DataGrid from a container div ID with JSON options
     * Creates canvases automatically inside the div
     * @param {string} container_id
     * @param {string} options_json
     * @returns {DataGrid}
     */
    static from_container(container_id, options_json) {
        const ptr0 = passStringToWasm0(container_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(options_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_from_container(ptr0, len0, ptr1, len1);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return DataGrid.__wrap(ret[0]);
    }
    /**
     * Create a new DataGrid instance with two canvas IDs (WebGL and text overlay)
     * @param {string} webgl_canvas_id
     * @param {string} text_canvas_id
     * @param {number} rows
     * @param {number} cols
     */
    constructor(webgl_canvas_id, text_canvas_id, rows, cols) {
        const ptr0 = passStringToWasm0(webgl_canvas_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(text_canvas_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_new(ptr0, len0, ptr1, len1, rows, cols);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        DataGridFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Render the grid
     */
    render() {
        wasm.datagrid_render(this.__wbg_ptr);
    }
    /**
     * Resize the grid
     * @param {number} width
     * @param {number} height
     */
    resize(width, height) {
        wasm.datagrid_resize(this.__wbg_ptr, width, height);
    }
    /**
     * Handle mouse wheel event for scrolling
     * @param {WheelEvent} event
     */
    handle_wheel(event) {
        wasm.datagrid_handle_wheel(this.__wbg_ptr, event);
    }
    /**
     * Handle mouse down event with modifier keys
     * @param {MouseEvent} event
     * @param {boolean} shift
     * @param {boolean} ctrl
     */
    handle_mouse_down_with_modifiers(event, shift, ctrl) {
        wasm.datagrid_handle_mouse_down_with_modifiers(this.__wbg_ptr, event, shift, ctrl);
    }
    /**
     * Handle mouse down event (legacy, for backward compatibility)
     * @param {MouseEvent} event
     */
    handle_mouse_down(event) {
        wasm.datagrid_handle_mouse_down(this.__wbg_ptr, event);
    }
    /**
     * Handle mouse down at coordinates with modifier keys (for JavaScript)
     * @param {number} x
     * @param {number} y
     * @param {boolean} shift
     * @param {boolean} ctrl
     */
    handle_mouse_down_at_with_modifiers(x, y, shift, ctrl) {
        wasm.datagrid_handle_mouse_down_at_with_modifiers(this.__wbg_ptr, x, y, shift, ctrl);
    }
    /**
     * Handle mouse down at specific coordinates
     * @param {number} x
     * @param {number} y
     */
    handle_mouse_down_at(x, y) {
        wasm.datagrid_handle_mouse_down_at(this.__wbg_ptr, x, y);
    }
    /**
     * Handle mouse up event
     * @param {MouseEvent} _event
     */
    handle_mouse_up(_event) {
        wasm.datagrid_handle_mouse_up(this.__wbg_ptr, _event);
    }
    /**
     * Handle mouse up at specific coordinates
     * @param {number} _x
     * @param {number} _y
     */
    handle_mouse_up_at(_x, _y) {
        wasm.datagrid_handle_mouse_up_at(this.__wbg_ptr, _x, _y);
    }
    /**
     * Handle mouse move event
     * @param {MouseEvent} event
     */
    handle_mouse_move(event) {
        wasm.datagrid_handle_mouse_move(this.__wbg_ptr, event);
    }
    /**
     * Handle context menu (right-click) event
     * Returns JSON with context info: {"type": "row"|"column"|"cell", "row": N, "col": N}
     * Returns empty string if not on grid
     * @param {MouseEvent} event
     * @returns {string}
     */
    handle_context_menu(event) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_handle_context_menu(this.__wbg_ptr, event);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get row operations for context menu
     * Returns available operations for the given row
     * @param {number} row
     * @returns {string[]}
     */
    get_row_context_operations(row) {
        const ret = wasm.datagrid_get_row_context_operations(this.__wbg_ptr, row);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * Execute row context menu operation
     * @param {string} operation
     * @param {number} row
     * @returns {string}
     */
    execute_row_operation(operation, row) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(operation, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.datagrid_execute_row_operation(this.__wbg_ptr, ptr0, len0, row);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * Set cell value
     * @param {number} row
     * @param {number} col
     * @param {string} value
     */
    set_cell_value(row, col, value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.datagrid_set_cell_value(this.__wbg_ptr, row, col, ptr0, len0);
    }
    /**
     * Get cell value
     * @param {number} row
     * @param {number} col
     * @returns {string}
     */
    get_cell_value(row, col) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_get_cell_value(this.__wbg_ptr, row, col);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get grid dimensions
     * @returns {Uint32Array}
     */
    get_dimensions() {
        const ret = wasm.datagrid_get_dimensions(this.__wbg_ptr);
        var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * Get viewport info
     * @returns {string}
     */
    get_viewport_info() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_get_viewport_info(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get visible cell range for lazy loading (returns [first_row, last_row, first_col, last_col])
     * @returns {Uint32Array}
     */
    get_visible_range() {
        const ret = wasm.datagrid_get_visible_range(this.__wbg_ptr);
        var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * Get viewport information as JSON string
     * Returns: "[canvas_width, canvas_height, scroll_y, scroll_x]"
     * @returns {string}
     */
    get_viewport_info_array() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_get_viewport_info_array(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get maximum scroll values as JSON string
     * Returns: "[max_scroll_x, max_scroll_y]"
     * @returns {string}
     */
    get_max_scroll() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_get_max_scroll(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get total content size (including headers) as JSON string
     * Returns: "[total_width, total_height]"
     * @returns {string}
     */
    get_total_size() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_get_total_size(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Set scroll position
     * @param {number} x
     * @param {number} y
     */
    set_scroll(x, y) {
        wasm.datagrid_set_scroll(this.__wbg_ptr, x, y);
    }
    /**
     * Set multiple cell values at once (for lazy loading/batch updates)
     * Takes JSON array of [row, col, value_type, value_data]
     * value_type: 0=empty, 1=text, 2=number, 3=boolean
     * Example: "[[0, 0, 1, \"text\"], [1, 1, 2, \"123\"]]"
     * @param {string} cells_data_json
     */
    set_cells_batch(cells_data_json) {
        const ptr0 = passStringToWasm0(cells_data_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_set_cells_batch(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Load grid data from JSON
     * Accepts JSON array: [{"row": 0, "col": 0, "value": "text"}, ...]
     * Value can be string, number, boolean, date, or null (for empty)
     * If column has data_type configured, value will be converted accordingly
     * @param {string} data_json
     */
    load_data_json(data_json) {
        const ptr0 = passStringToWasm0(data_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_load_data_json(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Load data for a specific range (for lazy loading)
     * Returns true if data is already loaded, false if needs loading
     * @param {number} start_row
     * @param {number} end_row
     * @param {number} start_col
     * @param {number} end_col
     * @returns {boolean}
     */
    is_range_loaded(start_row, end_row, start_col, end_col) {
        const ret = wasm.datagrid_is_range_loaded(this.__wbg_ptr, start_row, end_row, start_col, end_col);
        return ret !== 0;
    }
    /**
     * Handle keyboard event
     * @param {KeyboardEvent} event
     * @returns {boolean}
     */
    handle_keyboard(event) {
        const ret = wasm.datagrid_handle_keyboard(this.__wbg_ptr, event);
        return ret !== 0;
    }
    /**
     * Handle keyboard event with modifier keys
     * @param {KeyboardEvent} event
     * @param {boolean} ctrl
     * @returns {boolean}
     */
    handle_keyboard_with_modifiers(event, ctrl) {
        const ret = wasm.datagrid_handle_keyboard_with_modifiers(this.__wbg_ptr, event, ctrl);
        return ret !== 0;
    }
    /**
     * Handle keyboard with key string and modifier flags (called from JavaScript)
     * @param {string} key
     * @param {boolean} ctrl
     * @param {boolean} shift
     * @returns {boolean}
     */
    handle_keyboard_with_modifiers_key(key, ctrl, shift) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_handle_keyboard_with_modifiers_key(this.__wbg_ptr, ptr0, len0, ctrl, shift);
        return ret !== 0;
    }
    /**
     * Start editing a cell (called from JavaScript)
     * @param {number} row
     * @param {number} col
     * @returns {boolean}
     */
    start_edit(row, col) {
        const ret = wasm.datagrid_start_edit(this.__wbg_ptr, row, col);
        return ret !== 0;
    }
    /**
     * End editing mode
     */
    end_edit() {
        wasm.datagrid_end_edit(this.__wbg_ptr);
    }
    /**
     * Check if currently editing
     * @returns {boolean}
     */
    is_editing() {
        const ret = wasm.datagrid_is_editing(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Update cell value during editing
     * @param {number} row
     * @param {number} col
     * @param {string} value
     */
    update_cell_value(row, col, value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.datagrid_update_cell_value(this.__wbg_ptr, row, col, ptr0, len0);
    }
    /**
     * Get cell position for editing (returns canvas coordinates)
     * @param {number} row
     * @param {number} col
     * @returns {Float32Array}
     */
    get_cell_edit_rect(row, col) {
        const ret = wasm.datagrid_get_cell_edit_rect(this.__wbg_ptr, row, col);
        var v1 = getArrayF32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * Handle double-click for editing
     * @param {MouseEvent} event
     * @returns {Uint32Array | undefined}
     */
    handle_double_click(event) {
        const ret = wasm.datagrid_handle_double_click(this.__wbg_ptr, event);
        let v1;
        if (ret[0] !== 0) {
            v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        }
        return v1;
    }
    /**
     * Check if mouse is over a resize handle
     * Returns: "col" for column resize, "row" for row resize, "none" otherwise
     * @param {number} x
     * @param {number} y
     * @returns {string}
     */
    check_resize_handle(x, y) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_check_resize_handle(this.__wbg_ptr, x, y);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Start column or row resize
     * @param {number} x
     * @param {number} y
     * @param {string} resize_type
     * @returns {boolean}
     */
    start_resize(x, y, resize_type) {
        const ptr0 = passStringToWasm0(resize_type, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_start_resize(this.__wbg_ptr, x, y, ptr0, len0);
        return ret !== 0;
    }
    /**
     * Update resize during drag
     * @param {number} x
     * @param {number} y
     */
    update_resize(x, y) {
        wasm.datagrid_update_resize(this.__wbg_ptr, x, y);
    }
    /**
     * End resize
     */
    end_resize() {
        wasm.datagrid_end_resize(this.__wbg_ptr);
    }
    /**
     * Check if currently resizing
     * @returns {boolean}
     */
    is_resizing() {
        const ret = wasm.datagrid_is_resizing(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Get selected cells as a JSON array of [row, col] pairs
     * Returns: "[[row1, col1], [row2, col2], ...]"
     * @returns {string}
     */
    get_selected_cells() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_get_selected_cells(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get selection count
     * @returns {number}
     */
    get_selection_count() {
        const ret = wasm.datagrid_get_selection_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Select all cells (Ctrl+A)
     */
    select_all() {
        wasm.datagrid_select_all(this.__wbg_ptr);
    }
    /**
     * Select entire row
     * @param {number} row
     */
    select_row(row) {
        wasm.datagrid_select_row(this.__wbg_ptr, row);
    }
    /**
     * Select entire column
     * @param {number} col
     */
    select_col(col) {
        wasm.datagrid_select_col(this.__wbg_ptr, col);
    }
    /**
     * Copy selected cells to TSV (Tab-Separated Values) format
     * Returns a string with cells separated by tabs and rows separated by newlines
     * @returns {string}
     */
    copy_selected_cells() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_copy_selected_cells(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Cut selected cells (copy and then clear)
     * @returns {string}
     */
    cut_selected_cells() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_cut_selected_cells(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Paste cells from TSV (Tab-Separated Values) format
     * Pastes starting from the current focus cell
     * @param {string} tsv_text
     */
    paste_cells(tsv_text) {
        const ptr0 = passStringToWasm0(tsv_text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_paste_cells(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Set background color for a cell (RGBA as u32: 0xRRGGBBAA)
     * @param {number} row
     * @param {number} col
     * @param {number} color
     */
    set_cell_bg_color(row, col, color) {
        wasm.datagrid_set_cell_bg_color(this.__wbg_ptr, row, col, color);
    }
    /**
     * Set foreground (text) color for a cell (RGBA as u32: 0xRRGGBBAA)
     * @param {number} row
     * @param {number} col
     * @param {number} color
     */
    set_cell_fg_color(row, col, color) {
        wasm.datagrid_set_cell_fg_color(this.__wbg_ptr, row, col, color);
    }
    /**
     * Set font style for a cell
     * @param {number} row
     * @param {number} col
     * @param {boolean} bold
     * @param {boolean} italic
     */
    set_cell_font_style(row, col, bold, italic) {
        wasm.datagrid_set_cell_font_style(this.__wbg_ptr, row, col, bold, italic);
    }
    /**
     * Clear background color for a cell
     * @param {number} row
     * @param {number} col
     */
    clear_cell_bg_color(row, col) {
        wasm.datagrid_clear_cell_bg_color(this.__wbg_ptr, row, col);
    }
    /**
     * Clear foreground color for a cell
     * @param {number} row
     * @param {number} col
     */
    clear_cell_fg_color(row, col) {
        wasm.datagrid_clear_cell_fg_color(this.__wbg_ptr, row, col);
    }
    /**
     * Set cell style (background, foreground, font) in one call
     * @param {number} row
     * @param {number} col
     * @param {number | null | undefined} bg_color
     * @param {number | null | undefined} fg_color
     * @param {boolean} bold
     * @param {boolean} italic
     */
    set_cell_style(row, col, bg_color, fg_color, bold, italic) {
        wasm.datagrid_set_cell_style(this.__wbg_ptr, row, col, isLikeNone(bg_color) ? 0x100000001 : (bg_color) >>> 0, isLikeNone(fg_color) ? 0x100000001 : (fg_color) >>> 0, bold, italic);
    }
    /**
     * Set custom border for a cell (top, right, bottom, or left)
     * side: 0=top, 1=right, 2=bottom, 3=left
     * @param {number} row
     * @param {number} col
     * @param {number} side
     * @param {number} color
     * @param {number} width
     */
    set_cell_border(row, col, side, color, width) {
        wasm.datagrid_set_cell_border(this.__wbg_ptr, row, col, side, color, width);
    }
    /**
     * Set all borders for a cell at once
     * @param {number} row
     * @param {number} col
     * @param {number} color
     * @param {number} width
     */
    set_cell_borders(row, col, color, width) {
        wasm.datagrid_set_cell_borders(this.__wbg_ptr, row, col, color, width);
    }
    /**
     * Clear border for a cell side
     * side: 0=top, 1=right, 2=bottom, 3=left, 4=all
     * @param {number} row
     * @param {number} col
     * @param {number} side
     */
    clear_cell_border(row, col, side) {
        wasm.datagrid_clear_cell_border(this.__wbg_ptr, row, col, side);
    }
    /**
     * Add a column group for multi-level headers
     * @param label - Group label text
     * @param start_col - First column in group (0-indexed)
     * @param end_col - Last column in group (0-indexed, inclusive)
     * @param level - Header level (0 = top level, 1 = second level, etc.)
     * @param {string} label
     * @param {number} start_col
     * @param {number} end_col
     * @param {number} level
     */
    add_column_group(label, start_col, end_col, level) {
        const ptr0 = passStringToWasm0(label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.datagrid_add_column_group(this.__wbg_ptr, ptr0, len0, start_col, end_col, level);
    }
    /**
     * Clear all column groups (revert to single-level headers)
     */
    clear_column_groups() {
        wasm.datagrid_clear_column_groups(this.__wbg_ptr);
    }
    /**
     * Set the height of each header row (default: 30px)
     * @param {number} height
     */
    set_header_row_height(height) {
        wasm.datagrid_set_header_row_height(this.__wbg_ptr, height);
    }
    /**
     * Get the current number of header levels
     * @returns {number}
     */
    get_header_levels() {
        const ret = wasm.datagrid_get_header_levels(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get total header height
     * @returns {number}
     */
    get_header_height() {
        const ret = wasm.datagrid_get_header_height(this.__wbg_ptr);
        return ret;
    }
    /**
     * Set validation pattern for a column
     * @param col - Column index (0-based)
     * @param pattern - JavaScript regex pattern (e.g., "^[0-9]+$" for numbers only)
     * @param message - Error message to display when validation fails
     * @param {number} col
     * @param {string} pattern
     * @param {string} message
     */
    set_column_validation(col, pattern, message) {
        const ptr0 = passStringToWasm0(pattern, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(message, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.datagrid_set_column_validation(this.__wbg_ptr, col, ptr0, len0, ptr1, len1);
    }
    /**
     * Clear validation pattern for a column
     * @param {number} col
     */
    clear_column_validation(col) {
        wasm.datagrid_clear_column_validation(this.__wbg_ptr, col);
    }
    /**
     * Get validation pattern and message for a column
     * Returns JSON string: {"pattern": "regex", "message": "error msg"} or empty string if no validation
     * @param {number} col
     * @returns {string}
     */
    get_column_validation(col) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_get_column_validation(this.__wbg_ptr, col);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Set whether a column is editable
     * @param col - Column index (0-based)
     * @param editable - true: editable, false: read-only
     * @param {number} col
     * @param {boolean} editable
     */
    set_column_editable(col, editable) {
        wasm.datagrid_set_column_editable(this.__wbg_ptr, col, editable);
    }
    /**
     * Check if a column is editable
     * @param {number} col
     * @returns {boolean}
     */
    is_column_editable(col) {
        const ret = wasm.datagrid_is_column_editable(this.__wbg_ptr, col);
        return ret !== 0;
    }
    /**
     * Get editable status for all columns as JSON array
     * Returns: "[true, false, true, ...]"
     * @returns {string}
     */
    get_all_column_editable_status() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_get_all_column_editable_status(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Set column header name
     * @param col - Column index (0-based)
     * @param name - Header name to display
     * @param {number} col
     * @param {string} name
     */
    set_column_name(col, name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.datagrid_set_column_name(this.__wbg_ptr, col, ptr0, len0);
    }
    /**
     * Insert a row at the specified position
     * @param {number} at_index
     */
    insert_row(at_index) {
        wasm.datagrid_insert_row(this.__wbg_ptr, at_index);
    }
    /**
     * Delete a row at the specified position
     * @param {number} index
     */
    delete_row(index) {
        wasm.datagrid_delete_row(this.__wbg_ptr, index);
    }
    /**
     * Delete multiple rows at once
     * @param indices - JSON array of row indices to delete, e.g., "[0, 2, 5]"
     * @param {string} indices_json
     */
    delete_rows(indices_json) {
        const ptr0 = passStringToWasm0(indices_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_delete_rows(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Get unique row indices from selected cells
     * Returns JSON array of row indices, e.g., "[0, 2, 5]"
     * @returns {string}
     */
    get_selected_row_indices() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_get_selected_row_indices(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Insert a column at the specified position
     * @param {number} at_index
     */
    insert_column(at_index) {
        wasm.datagrid_insert_column(this.__wbg_ptr, at_index);
    }
    /**
     * Delete a column at the specified position
     * @param {number} index
     */
    delete_column(index) {
        wasm.datagrid_delete_column(this.__wbg_ptr, index);
    }
    /**
     * Delete all empty rows (rows with no non-empty cells)
     * @returns {number}
     */
    delete_empty_rows() {
        const ret = wasm.datagrid_delete_empty_rows(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Check if a row is empty (all cells are empty)
     * @param {number} row
     * @returns {boolean}
     */
    is_row_empty(row) {
        const ret = wasm.datagrid_is_row_empty(this.__wbg_ptr, row);
        return ret !== 0;
    }
    /**
     * Find all modified (edited) cells
     * @returns {number}
     */
    find_modified_cells() {
        const ret = wasm.datagrid_find_modified_cells(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Clear modified flags from all cells
     */
    clear_all_modified_flags() {
        wasm.datagrid_clear_all_modified_flags(this.__wbg_ptr);
    }
    /**
     * Check if a cell is modified
     * @param {number} row
     * @param {number} col
     * @returns {boolean}
     */
    is_cell_modified(row, col) {
        const ret = wasm.datagrid_is_cell_modified(this.__wbg_ptr, row, col);
        return ret !== 0;
    }
    /**
     * Get count of modified cells
     * @returns {number}
     */
    get_modified_cell_count() {
        const ret = wasm.datagrid_get_modified_cell_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Search for text in grid cells (case-insensitive by default)
     * @param {string} query
     * @returns {number}
     */
    search_text(query) {
        const ptr0 = passStringToWasm0(query, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_search_text(this.__wbg_ptr, ptr0, len0);
        return ret >>> 0;
    }
    /**
     * Search for text with options
     * @param {string} query
     * @param {boolean} case_sensitive
     * @param {boolean} whole_word
     * @returns {number}
     */
    search_text_with_options(query, case_sensitive, whole_word) {
        const ptr0 = passStringToWasm0(query, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_search_text_with_options(this.__wbg_ptr, ptr0, len0, case_sensitive, whole_word);
        return ret >>> 0;
    }
    /**
     * Move to next search result
     * @returns {boolean}
     */
    search_next() {
        const ret = wasm.datagrid_search_next(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Move to previous search result
     * @returns {boolean}
     */
    search_prev() {
        const ret = wasm.datagrid_search_prev(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Search using regular expression
     * @param {string} pattern
     * @param {boolean} case_sensitive
     * @returns {number}
     */
    search_regex(pattern, case_sensitive) {
        const ptr0 = passStringToWasm0(pattern, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_search_regex(this.__wbg_ptr, ptr0, len0, case_sensitive);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * Validate regex pattern without performing search
     * @param {string} pattern
     * @returns {boolean}
     */
    validate_regex_pattern(pattern) {
        const ptr0 = passStringToWasm0(pattern, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_validate_regex_pattern(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * Clear search results
     */
    clear_search() {
        wasm.datagrid_clear_search(this.__wbg_ptr);
    }
    /**
     * Get search result count
     * @returns {number}
     */
    get_search_result_count() {
        const ret = wasm.datagrid_get_search_result_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get current search index (1-based for display)
     * @returns {number}
     */
    get_current_search_index() {
        const ret = wasm.datagrid_get_current_search_index(this.__wbg_ptr);
        return ret;
    }
    /**
     * Check if a cell is a search result
     * @param {number} row
     * @param {number} col
     * @returns {boolean}
     */
    is_search_result(row, col) {
        const ret = wasm.datagrid_is_search_result(this.__wbg_ptr, row, col);
        return ret !== 0;
    }
    /**
     * Check if a cell is the current (active) search result
     * @param {number} row
     * @param {number} col
     * @returns {boolean}
     */
    is_current_search_result(row, col) {
        const ret = wasm.datagrid_is_current_search_result(this.__wbg_ptr, row, col);
        return ret !== 0;
    }
    /**
     * Replace current search result with new text
     * @param {string} replacement
     * @returns {boolean}
     */
    replace_current(replacement) {
        const ptr0 = passStringToWasm0(replacement, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_replace_current(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * Replace all search results with new text
     * @param {string} replacement
     * @returns {number}
     */
    replace_all(replacement) {
        const ptr0 = passStringToWasm0(replacement, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_replace_all(this.__wbg_ptr, ptr0, len0);
        return ret >>> 0;
    }
    /**
     * Replace in selection only
     * @param {string} search
     * @param {string} replacement
     * @param {boolean} case_sensitive
     * @returns {number}
     */
    replace_in_selection(search, replacement, case_sensitive) {
        const ptr0 = passStringToWasm0(search, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(replacement, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_replace_in_selection(this.__wbg_ptr, ptr0, len0, ptr1, len1, case_sensitive);
        return ret >>> 0;
    }
    /**
     * Sort by column
     * @param {number} col
     * @param {boolean} ascending
     */
    sort_by_column(col, ascending) {
        wasm.datagrid_sort_by_column(this.__wbg_ptr, col, ascending);
    }
    /**
     * Toggle sort on column (click column header)
     * @param {number} col
     */
    toggle_column_sort(col) {
        wasm.datagrid_toggle_column_sort(this.__wbg_ptr, col);
    }
    /**
     * Get sort state for a column as JSON object
     * Returns: "{\"is_sorted\": true/false, \"is_ascending\": true/false}"
     * @param {number} col
     * @returns {string}
     */
    get_column_sort_state(col) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_get_column_sort_state(this.__wbg_ptr, col);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Add column to multi-column sort (for Shift+Click)
     * @param {number} col
     * @param {boolean} ascending
     */
    add_multi_column_sort(col, ascending) {
        wasm.datagrid_add_multi_column_sort(this.__wbg_ptr, col, ascending);
    }
    /**
     * Toggle column in multi-column sort
     * @param {number} col
     */
    toggle_multi_column_sort(col) {
        wasm.datagrid_toggle_multi_column_sort(this.__wbg_ptr, col);
    }
    /**
     * Clear multi-column sort
     */
    clear_multi_column_sort() {
        wasm.datagrid_clear_multi_column_sort(this.__wbg_ptr);
    }
    /**
     * Get multi-column sort state as JSON array of [col, ascending] pairs
     * Returns: "[[col1, 1], [col2, 0], ...]" where 1=ascending, 0=descending
     * @returns {string}
     */
    get_multi_column_sort_state() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_get_multi_column_sort_state(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Check if a column is in multi-column sort
     * Returns JSON: "{\"is_sorted\": bool, \"is_ascending\": bool, \"sort_priority\": number}"
     * @param {number} col
     * @returns {string}
     */
    get_column_multi_sort_state(col) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_get_column_multi_sort_state(this.__wbg_ptr, col);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Freeze first N rows
     * @param {number} count
     */
    freeze_rows(count) {
        wasm.datagrid_freeze_rows(this.__wbg_ptr, count);
    }
    /**
     * Freeze first N columns
     * @param {number} count
     */
    freeze_cols(count) {
        wasm.datagrid_freeze_cols(this.__wbg_ptr, count);
    }
    /**
     * Get frozen row count
     * @returns {number}
     */
    get_frozen_rows() {
        const ret = wasm.datagrid_get_frozen_rows(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Set frozen row count
     * @param {number} count
     */
    set_frozen_rows(count) {
        wasm.datagrid_set_frozen_rows(this.__wbg_ptr, count);
    }
    /**
     * Get frozen column count
     * @returns {number}
     */
    get_frozen_cols() {
        const ret = wasm.datagrid_get_frozen_cols(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Set frozen column count
     * @param {number} count
     */
    set_frozen_cols(count) {
        wasm.datagrid_set_frozen_cols(this.__wbg_ptr, count);
    }
    /**
     * Undo last edit action
     * @returns {boolean}
     */
    undo() {
        const ret = wasm.datagrid_undo(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Redo last undone action
     * @returns {boolean}
     */
    redo() {
        const ret = wasm.datagrid_redo(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Check if undo is available
     * @returns {boolean}
     */
    can_undo() {
        const ret = wasm.datagrid_can_undo(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Check if redo is available
     * @returns {boolean}
     */
    can_redo() {
        const ret = wasm.datagrid_can_redo(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Get undo stack size
     * @returns {number}
     */
    get_undo_count() {
        const ret = wasm.datagrid_get_undo_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get redo stack size
     * @returns {number}
     */
    get_redo_count() {
        const ret = wasm.datagrid_get_redo_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Auto-fit column width to content
     * @param {number} col
     */
    auto_fit_column(col) {
        wasm.datagrid_auto_fit_column(this.__wbg_ptr, col);
    }
    /**
     * Auto-fit all columns to content
     */
    auto_fit_all_columns() {
        wasm.datagrid_auto_fit_all_columns(this.__wbg_ptr);
    }
    /**
     * Set all columns to equal width
     * @param {number} width
     */
    set_all_columns_equal_width(width) {
        wasm.datagrid_set_all_columns_equal_width(this.__wbg_ptr, width);
    }
    /**
     * Filter column by text (case-insensitive contains)
     * @param {number} col
     * @param {string} text
     */
    filter_column_by_text(col, text) {
        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.datagrid_filter_column_by_text(this.__wbg_ptr, col, ptr0, len0);
    }
    /**
     * Filter column by empty cells
     * @param {number} col
     */
    filter_column_show_non_empty(col) {
        wasm.datagrid_filter_column_show_non_empty(this.__wbg_ptr, col);
    }
    /**
     * Clear all column filters
     */
    clear_column_filters() {
        wasm.datagrid_clear_column_filters(this.__wbg_ptr);
    }
    /**
     * Check if a row is filtered (hidden)
     * @param {number} row
     * @returns {boolean}
     */
    is_row_filtered(row) {
        const ret = wasm.datagrid_is_row_filtered(this.__wbg_ptr, row);
        return ret !== 0;
    }
    /**
     * Get visible row count
     * @returns {number}
     */
    get_visible_row_count() {
        const ret = wasm.datagrid_get_visible_row_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Start performance benchmark (returns start time)
     * @returns {number}
     */
    benchmark_start() {
        const ret = wasm.datagrid_benchmark_start(this.__wbg_ptr);
        return ret;
    }
    /**
     * End performance benchmark and return elapsed time in ms
     * @param {number} start_time
     * @returns {number}
     */
    benchmark_end(start_time) {
        const ret = wasm.datagrid_benchmark_end(this.__wbg_ptr, start_time);
        return ret;
    }
    /**
     * Update FPS tracking (call this in render loop)
     */
    update_performance_metrics() {
        wasm.datagrid_update_performance_metrics(this.__wbg_ptr);
    }
    /**
     * Get current FPS
     * @returns {number}
     */
    get_fps() {
        const ret = wasm.datagrid_get_fps(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get total frame count
     * @returns {number}
     */
    get_frame_count() {
        const ret = wasm.datagrid_get_frame_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get last render time in ms
     * @returns {number}
     */
    get_last_render_time() {
        const ret = wasm.datagrid_get_last_render_time(this.__wbg_ptr);
        return ret;
    }
    /**
     * Reset performance metrics
     */
    reset_performance_metrics() {
        wasm.datagrid_reset_performance_metrics(this.__wbg_ptr);
    }
    /**
     * Run performance benchmark (render N frames and return average time)
     * @param {number} frame_count
     * @returns {number}
     */
    run_benchmark(frame_count) {
        const ret = wasm.datagrid_run_benchmark(this.__wbg_ptr, frame_count);
        return ret;
    }
    /**
     * Mark a specific cell as dirty (needs re-rendering)
     * @param {number} row
     * @param {number} col
     */
    mark_cell_dirty(row, col) {
        wasm.datagrid_mark_cell_dirty(this.__wbg_ptr, row, col);
    }
    /**
     * Mark all cells as dirty (force full re-render)
     */
    mark_all_dirty() {
        wasm.datagrid_mark_all_dirty(this.__wbg_ptr);
    }
    /**
     * Clear dirty cells (after rendering)
     */
    clear_dirty_cells() {
        wasm.datagrid_clear_dirty_cells(this.__wbg_ptr);
    }
    /**
     * Get count of dirty cells
     * @returns {number}
     */
    get_dirty_cell_count() {
        const ret = wasm.datagrid_get_dirty_cell_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Check if full render is needed
     * @returns {boolean}
     */
    needs_full_render() {
        const ret = wasm.datagrid_needs_full_render(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Optimize memory by reserving capacity for expected data size
     * @param {number} expected_cells
     */
    reserve_capacity(expected_cells) {
        wasm.datagrid_reserve_capacity(this.__wbg_ptr, expected_cells);
    }
    /**
     * Clear all non-essential cached data to free memory
     */
    clear_caches() {
        wasm.datagrid_clear_caches(this.__wbg_ptr);
    }
    /**
     * Get estimated memory usage in bytes (approximate)
     * @returns {number}
     */
    get_memory_usage() {
        const ret = wasm.datagrid_get_memory_usage(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Compact memory by removing unused allocations
     */
    compact_memory() {
        wasm.datagrid_compact_memory(this.__wbg_ptr);
    }
    /**
     * Export grid data as JSON for worker thread processing
     * Returns JSON array: [{"row":0,"col":0,"value":"text","type":"text"}, ...]
     * @returns {string}
     */
    export_grid_data_json() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_export_grid_data_json(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Export a specific range of data as JSON for worker processing
     * Returns JSON array for the specified range
     * @param {number} start_row
     * @param {number} end_row
     * @param {number} start_col
     * @param {number} end_col
     * @returns {string}
     */
    export_range_json(start_row, end_row, start_col, end_col) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_export_range_json(this.__wbg_ptr, start_row, end_row, start_col, end_col);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Import processed data from worker thread
     * Accepts JSON array: [{"row":0,"col":0,"value":"text","type":"text"}, ...]
     * @param {string} result_json
     * @returns {number}
     */
    import_worker_result(result_json) {
        const ptr0 = passStringToWasm0(result_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_import_worker_result(this.__wbg_ptr, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * Get grid metadata for worker thread (dimensions, frozen areas, etc.)
     * @returns {string}
     */
    get_grid_metadata_json() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.datagrid_get_grid_metadata_json(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Prepare data for sorting in worker thread
     * Returns JSON with data and sort configuration
     * sort_columns_json: JSON array of column indices, e.g. "[0, 1]"
     * ascending_json: JSON array of booleans, e.g. "[true, false]"
     * @param {string} sort_columns_json
     * @param {string} ascending_json
     * @returns {string}
     */
    prepare_sort_data(sort_columns_json, ascending_json) {
        let deferred4_0;
        let deferred4_1;
        try {
            const ptr0 = passStringToWasm0(sort_columns_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(ascending_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const ret = wasm.datagrid_prepare_sort_data(this.__wbg_ptr, ptr0, len0, ptr1, len1);
            var ptr3 = ret[0];
            var len3 = ret[1];
            if (ret[3]) {
                ptr3 = 0; len3 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred4_0 = ptr3;
            deferred4_1 = len3;
            return getStringFromWasm0(ptr3, len3);
        } finally {
            wasm.__wbindgen_free(deferred4_0, deferred4_1, 1);
        }
    }
    /**
     * Apply sorted row indices from worker result
     * Takes array of row indices representing the new order
     * @param {string} indices_json
     */
    apply_sorted_indices(indices_json) {
        const ptr0 = passStringToWasm0(indices_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datagrid_apply_sorted_indices(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) DataGrid.prototype[Symbol.dispose] = DataGrid.prototype.free;

const EXPECTED_RESPONSE_TYPES = new Set(['basic', 'cors', 'default']);

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                const validResponse = module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg___wbindgen_boolean_get_6d5a1ee65bab5f68 = function(arg0) {
        const v = arg0;
        const ret = typeof(v) === 'boolean' ? v : undefined;
        return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
    };
    imports.wbg.__wbg___wbindgen_is_undefined_2d472862bd29a478 = function(arg0) {
        const ret = arg0 === undefined;
        return ret;
    };
    imports.wbg.__wbg___wbindgen_throw_b855445ff6a94295 = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_appendChild_aec7a8a4bd6cac61 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.appendChild(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_attachShader_28ab04bfd0eeb19d = function(arg0, arg1, arg2) {
        arg0.attachShader(arg1, arg2);
    };
    imports.wbg.__wbg_beginPath_ae4169e263573dcd = function(arg0) {
        arg0.beginPath();
    };
    imports.wbg.__wbg_bindBuffer_3c6f3ecc1a210ca3 = function(arg0, arg1, arg2) {
        arg0.bindBuffer(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_blendFunc_36124869ea5b36ae = function(arg0, arg1, arg2) {
        arg0.blendFunc(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_bufferData_6c7fa43be0e969d6 = function(arg0, arg1, arg2, arg3) {
        arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
    };
    imports.wbg.__wbg_call_e762c39fa8ea36bf = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.call(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clearColor_95a9ab5565d42083 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.clearColor(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_clearRect_155b2e12f737565e = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.clearRect(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_clear_21e859b27ff741c4 = function(arg0, arg1) {
        arg0.clear(arg1 >>> 0);
    };
    imports.wbg.__wbg_clip_7858b458fb895725 = function(arg0) {
        arg0.clip();
    };
    imports.wbg.__wbg_compileShader_8be7809a35b5b8d1 = function(arg0, arg1) {
        arg0.compileShader(arg1);
    };
    imports.wbg.__wbg_createBuffer_9ec61509720be784 = function(arg0) {
        const ret = arg0.createBuffer();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createElement_964ab674a0176cd8 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_createProgram_3de15304f8ebbc28 = function(arg0) {
        const ret = arg0.createProgram();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createShader_800924f280388e4d = function(arg0, arg1) {
        const ret = arg0.createShader(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_deltaX_52dbec35cfc88ef2 = function(arg0) {
        const ret = arg0.deltaX;
        return ret;
    };
    imports.wbg.__wbg_deltaY_533a14decfb96f6b = function(arg0) {
        const ret = arg0.deltaY;
        return ret;
    };
    imports.wbg.__wbg_document_725ae06eb442a6db = function(arg0) {
        const ret = arg0.document;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_drawArrays_268fca68622dd23f = function(arg0, arg1, arg2, arg3) {
        arg0.drawArrays(arg1 >>> 0, arg2, arg3);
    };
    imports.wbg.__wbg_enableVertexAttribArray_a2d36c7d18a4a692 = function(arg0, arg1) {
        arg0.enableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__wbg_enable_3c4fab29e1f03b55 = function(arg0, arg1) {
        arg0.enable(arg1 >>> 0);
    };
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_fillRect_726041755e54e83d = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.fillRect(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_fillText_c2ae7e4487ec82dd = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        arg0.fillText(getStringFromWasm0(arg1, arg2), arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_getAttribLocation_b544bb90d1c65c92 = function(arg0, arg1, arg2, arg3) {
        const ret = arg0.getAttribLocation(arg1, getStringFromWasm0(arg2, arg3));
        return ret;
    };
    imports.wbg.__wbg_getContext_0b80ccb9547db509 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments) };
    imports.wbg.__wbg_getElementById_c365dd703c4a88c3 = function(arg0, arg1, arg2) {
        const ret = arg0.getElementById(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_getProgramInfoLog_ce6f5e0603a4927f = function(arg0, arg1, arg2) {
        const ret = arg1.getProgramInfoLog(arg2);
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_getProgramParameter_9e84a8e91d9bd349 = function(arg0, arg1, arg2) {
        const ret = arg0.getProgramParameter(arg1, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_getShaderInfoLog_8802198fabe2d112 = function(arg0, arg1, arg2) {
        const ret = arg1.getShaderInfoLog(arg2);
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_getShaderParameter_f7a968e7357add60 = function(arg0, arg1, arg2) {
        const ret = arg0.getShaderParameter(arg1, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_getUniformLocation_eec60dd414033654 = function(arg0, arg1, arg2, arg3) {
        const ret = arg0.getUniformLocation(arg1, getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_height_119077665279308c = function(arg0) {
        const ret = arg0.height;
        return ret;
    };
    imports.wbg.__wbg_instanceof_CanvasRenderingContext2d_c0728747cf1e699c = function(arg0) {
        let result;
        try {
            result = arg0 instanceof CanvasRenderingContext2D;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlCanvasElement_3e2e95b109dae976 = function(arg0) {
        let result;
        try {
            result = arg0 instanceof HTMLCanvasElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_WebGlRenderingContext_29ac37f0cb7afc9b = function(arg0) {
        let result;
        try {
            result = arg0 instanceof WebGLRenderingContext;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Window_4846dbb3de56c84c = function(arg0) {
        let result;
        try {
            result = arg0 instanceof Window;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_key_32aa43e1cae08d29 = function(arg0, arg1) {
        const ret = arg1.key;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_linkProgram_95ada1a5ea318894 = function(arg0, arg1) {
        arg0.linkProgram(arg1);
    };
    imports.wbg.__wbg_log_8cec76766b8c0e33 = function(arg0) {
        console.log(arg0);
    };
    imports.wbg.__wbg_measureText_d63127eb84829830 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.measureText(getStringFromWasm0(arg1, arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_new_no_args_ee98eee5275000a4 = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_now_f5ba683d8ce2c571 = function(arg0) {
        const ret = arg0.now();
        return ret;
    };
    imports.wbg.__wbg_offsetX_4bd247aa56ff346f = function(arg0) {
        const ret = arg0.offsetX;
        return ret;
    };
    imports.wbg.__wbg_offsetY_2edf7781ad0674a1 = function(arg0) {
        const ret = arg0.offsetY;
        return ret;
    };
    imports.wbg.__wbg_performance_e8315b5ae987e93f = function(arg0) {
        const ret = arg0.performance;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_rect_d2677b1857072f26 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.rect(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_restore_9e6a0f2c35799ecd = function(arg0) {
        arg0.restore();
    };
    imports.wbg.__wbg_save_62f4925fcc246f6c = function(arg0) {
        arg0.save();
    };
    imports.wbg.__wbg_setAttribute_9bad76f39609daac = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        arg0.setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_set_fillStyle_2bb9477555f866ec = function(arg0, arg1) {
        arg0.fillStyle = arg1;
    };
    imports.wbg.__wbg_set_font_bd9a29cab7b9db0c = function(arg0, arg1, arg2) {
        arg0.font = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_set_height_89110f48f7fd0817 = function(arg0, arg1) {
        arg0.height = arg1 >>> 0;
    };
    imports.wbg.__wbg_set_innerHTML_fb5a7e25198fc344 = function(arg0, arg1, arg2) {
        arg0.innerHTML = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_set_lineWidth_4059ac6bb1d807f8 = function(arg0, arg1) {
        arg0.lineWidth = arg1;
    };
    imports.wbg.__wbg_set_strokeStyle_4d716c7d92c95b45 = function(arg0, arg1) {
        arg0.strokeStyle = arg1;
    };
    imports.wbg.__wbg_set_textAlign_e2202d9a7471d2d0 = function(arg0, arg1, arg2) {
        arg0.textAlign = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_set_textBaseline_73dbeaf15e2bb1bf = function(arg0, arg1, arg2) {
        arg0.textBaseline = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_set_width_dcc02c61dd01cff6 = function(arg0, arg1) {
        arg0.width = arg1 >>> 0;
    };
    imports.wbg.__wbg_shaderSource_328f9044e2c98a85 = function(arg0, arg1, arg2, arg3) {
        arg0.shaderSource(arg1, getStringFromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_shiftKey_e0b189884cc0d006 = function(arg0) {
        const ret = arg0.shiftKey;
        return ret;
    };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_89e1d9ac6a1b250e = function() {
        const ret = typeof global === 'undefined' ? null : global;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_THIS_8b530f326a9e48ac = function() {
        const ret = typeof globalThis === 'undefined' ? null : globalThis;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_SELF_6fdf4b64710cc91b = function() {
        const ret = typeof self === 'undefined' ? null : self;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_WINDOW_b45bfc5a37f6cfa2 = function() {
        const ret = typeof window === 'undefined' ? null : window;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_strokeRect_788876bb2e67b691 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.strokeRect(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_uniform2f_587619767a15ed7e = function(arg0, arg1, arg2, arg3) {
        arg0.uniform2f(arg1, arg2, arg3);
    };
    imports.wbg.__wbg_useProgram_3cc28f936528f842 = function(arg0, arg1) {
        arg0.useProgram(arg1);
    };
    imports.wbg.__wbg_vertexAttribPointer_4c4826c855c381d0 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        arg0.vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    };
    imports.wbg.__wbg_viewport_6e8b657130b529c0 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.viewport(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_width_619a651232e6844f = function(arg0) {
        const ret = arg0.width;
        return ret;
    };
    imports.wbg.__wbg_width_9ea2df52b5d2c909 = function(arg0) {
        const ret = arg0.width;
        return ret;
    };
    imports.wbg.__wbindgen_cast_2241b6af4c4b2941 = function(arg0, arg1) {
        // Cast intrinsic for `Ref(String) -> Externref`.
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_cast_cd07b1914aa3d62c = function(arg0, arg1) {
        // Cast intrinsic for `Ref(Slice(F32)) -> NamedExternref("Float32Array")`.
        const ret = getArrayF32FromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_externrefs;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };

    return imports;
}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedFloat32ArrayMemory0 = null;
    cachedUint32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('datagrid5_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;

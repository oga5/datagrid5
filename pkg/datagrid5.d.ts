/* tslint:disable */
/* eslint-disable */
/**
 * Initialize the library
 */
export function init(): void;
/**
 * Main DataGrid control
 */
export class DataGrid {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Create a new DataGrid from a container div ID with JSON options
   * Creates canvases automatically inside the div
   */
  static from_container(container_id: string, options_json: string): DataGrid;
  /**
   * Create a new DataGrid instance with two canvas IDs (WebGL and text overlay)
   */
  constructor(webgl_canvas_id: string, text_canvas_id: string, rows: number, cols: number);
  /**
   * Render the grid
   */
  render(): void;
  /**
   * Resize the grid
   */
  resize(width: number, height: number): void;
  /**
   * Handle mouse wheel event for scrolling
   */
  handle_wheel(event: WheelEvent): void;
  /**
   * Handle mouse down event with modifier keys
   */
  handle_mouse_down_with_modifiers(event: MouseEvent, shift: boolean, ctrl: boolean): void;
  /**
   * Handle mouse down event (legacy, for backward compatibility)
   */
  handle_mouse_down(event: MouseEvent): void;
  /**
   * Handle mouse down at coordinates with modifier keys (for JavaScript)
   */
  handle_mouse_down_at_with_modifiers(x: number, y: number, shift: boolean, ctrl: boolean): void;
  /**
   * Handle mouse down at specific coordinates
   */
  handle_mouse_down_at(x: number, y: number): void;
  /**
   * Handle mouse up event
   */
  handle_mouse_up(_event: MouseEvent): void;
  /**
   * Handle mouse up at specific coordinates
   */
  handle_mouse_up_at(_x: number, _y: number): void;
  /**
   * Handle mouse move event
   */
  handle_mouse_move(event: MouseEvent): void;
  /**
   * Handle context menu (right-click) event
   * Returns JSON with context info: {"type": "row"|"column"|"cell", "row": N, "col": N}
   * Returns empty string if not on grid
   */
  handle_context_menu(event: MouseEvent): string;
  /**
   * Get row operations for context menu
   * Returns available operations for the given row
   */
  get_row_context_operations(row: number): string[];
  /**
   * Execute row context menu operation
   */
  execute_row_operation(operation: string, row: number): string;
  /**
   * Set cell value
   */
  set_cell_value(row: number, col: number, value: string): void;
  /**
   * Get cell value
   */
  get_cell_value(row: number, col: number): string;
  /**
   * Get grid dimensions
   */
  get_dimensions(): Uint32Array;
  /**
   * Get viewport info
   */
  get_viewport_info(): string;
  /**
   * Get visible cell range for lazy loading (returns [first_row, last_row, first_col, last_col])
   */
  get_visible_range(): Uint32Array;
  /**
   * Get viewport information as JSON string
   * Returns: "[canvas_width, canvas_height, scroll_y, scroll_x]"
   */
  get_viewport_info_array(): string;
  /**
   * Get maximum scroll values as JSON string
   * Returns: "[max_scroll_x, max_scroll_y]"
   */
  get_max_scroll(): string;
  /**
   * Get total content size (including headers) as JSON string
   * Returns: "[total_width, total_height]"
   */
  get_total_size(): string;
  /**
   * Set scroll position
   */
  set_scroll(x: number, y: number): void;
  /**
   * Set multiple cell values at once (for lazy loading/batch updates)
   * Takes JSON array of [row, col, value_type, value_data]
   * value_type: 0=empty, 1=text, 2=number, 3=boolean
   * Example: "[[0, 0, 1, \"text\"], [1, 1, 2, \"123\"]]"
   */
  set_cells_batch(cells_data_json: string): void;
  /**
   * Load grid data from JSON
   * Accepts JSON array: [{"row": 0, "col": 0, "value": "text"}, ...]
   * Value can be string, number, boolean, date, or null (for empty)
   * If column has data_type configured, value will be converted accordingly
   */
  load_data_json(data_json: string): void;
  /**
   * Load data for a specific range (for lazy loading)
   * Returns true if data is already loaded, false if needs loading
   */
  is_range_loaded(start_row: number, end_row: number, start_col: number, end_col: number): boolean;
  /**
   * Handle keyboard event
   */
  handle_keyboard(event: KeyboardEvent): boolean;
  /**
   * Handle keyboard event with modifier keys
   */
  handle_keyboard_with_modifiers(event: KeyboardEvent, ctrl: boolean): boolean;
  /**
   * Handle keyboard with key string and modifier flags (called from JavaScript)
   */
  handle_keyboard_with_modifiers_key(key: string, ctrl: boolean, shift: boolean): boolean;
  /**
   * Start editing a cell (called from JavaScript)
   */
  start_edit(row: number, col: number): boolean;
  /**
   * End editing mode
   */
  end_edit(): void;
  /**
   * Check if currently editing
   */
  is_editing(): boolean;
  /**
   * Update cell value during editing
   */
  update_cell_value(row: number, col: number, value: string): void;
  /**
   * Get cell position for editing (returns canvas coordinates)
   */
  get_cell_edit_rect(row: number, col: number): Float32Array;
  /**
   * Handle double-click for editing
   */
  handle_double_click(event: MouseEvent): Uint32Array | undefined;
  /**
   * Check if mouse is over a resize handle
   * Returns: "col" for column resize, "row" for row resize, "none" otherwise
   */
  check_resize_handle(x: number, y: number): string;
  /**
   * Start column or row resize
   */
  start_resize(x: number, y: number, resize_type: string): boolean;
  /**
   * Update resize during drag
   */
  update_resize(x: number, y: number): void;
  /**
   * End resize
   */
  end_resize(): void;
  /**
   * Check if currently resizing
   */
  is_resizing(): boolean;
  /**
   * Get selected cells as a JSON array of [row, col] pairs
   * Returns: "[[row1, col1], [row2, col2], ...]"
   */
  get_selected_cells(): string;
  /**
   * Get selection count
   */
  get_selection_count(): number;
  /**
   * Select all cells (Ctrl+A)
   */
  select_all(): void;
  /**
   * Select entire row
   */
  select_row(row: number): void;
  /**
   * Select entire column
   */
  select_col(col: number): void;
  /**
   * Copy selected cells to TSV (Tab-Separated Values) format
   * Returns a string with cells separated by tabs and rows separated by newlines
   */
  copy_selected_cells(): string;
  /**
   * Cut selected cells (copy and then clear)
   */
  cut_selected_cells(): string;
  /**
   * Paste cells from TSV (Tab-Separated Values) format
   * Pastes starting from the current focus cell
   */
  paste_cells(tsv_text: string): void;
  /**
   * Set background color for a cell (RGBA as u32: 0xRRGGBBAA)
   */
  set_cell_bg_color(row: number, col: number, color: number): void;
  /**
   * Set foreground (text) color for a cell (RGBA as u32: 0xRRGGBBAA)
   */
  set_cell_fg_color(row: number, col: number, color: number): void;
  /**
   * Set font style for a cell
   */
  set_cell_font_style(row: number, col: number, bold: boolean, italic: boolean): void;
  /**
   * Clear background color for a cell
   */
  clear_cell_bg_color(row: number, col: number): void;
  /**
   * Clear foreground color for a cell
   */
  clear_cell_fg_color(row: number, col: number): void;
  /**
   * Set cell style (background, foreground, font) in one call
   */
  set_cell_style(row: number, col: number, bg_color: number | null | undefined, fg_color: number | null | undefined, bold: boolean, italic: boolean): void;
  /**
   * Set custom border for a cell (top, right, bottom, or left)
   * side: 0=top, 1=right, 2=bottom, 3=left
   */
  set_cell_border(row: number, col: number, side: number, color: number, width: number): void;
  /**
   * Set all borders for a cell at once
   */
  set_cell_borders(row: number, col: number, color: number, width: number): void;
  /**
   * Clear border for a cell side
   * side: 0=top, 1=right, 2=bottom, 3=left, 4=all
   */
  clear_cell_border(row: number, col: number, side: number): void;
  /**
   * Add a column group for multi-level headers
   * @param label - Group label text
   * @param start_col - First column in group (0-indexed)
   * @param end_col - Last column in group (0-indexed, inclusive)
   * @param level - Header level (0 = top level, 1 = second level, etc.)
   */
  add_column_group(label: string, start_col: number, end_col: number, level: number): void;
  /**
   * Clear all column groups (revert to single-level headers)
   */
  clear_column_groups(): void;
  /**
   * Set the height of each header row (default: 30px)
   */
  set_header_row_height(height: number): void;
  /**
   * Get the current number of header levels
   */
  get_header_levels(): number;
  /**
   * Get total header height
   */
  get_header_height(): number;
  /**
   * Set validation pattern for a column
   * @param col - Column index (0-based)
   * @param pattern - JavaScript regex pattern (e.g., "^[0-9]+$" for numbers only)
   * @param message - Error message to display when validation fails
   */
  set_column_validation(col: number, pattern: string, message: string): void;
  /**
   * Clear validation pattern for a column
   */
  clear_column_validation(col: number): void;
  /**
   * Get validation pattern and message for a column
   * Returns JSON string: {"pattern": "regex", "message": "error msg"} or empty string if no validation
   */
  get_column_validation(col: number): string;
  /**
   * Set whether a column is editable
   * @param col - Column index (0-based)
   * @param editable - true: editable, false: read-only
   */
  set_column_editable(col: number, editable: boolean): void;
  /**
   * Check if a column is editable
   */
  is_column_editable(col: number): boolean;
  /**
   * Get editable status for all columns as JSON array
   * Returns: "[true, false, true, ...]"
   */
  get_all_column_editable_status(): string;
  /**
   * Set column header name
   * @param col - Column index (0-based)
   * @param name - Header name to display
   */
  set_column_name(col: number, name: string): void;
  /**
   * Insert a row at the specified position
   */
  insert_row(at_index: number): void;
  /**
   * Delete a row at the specified position
   */
  delete_row(index: number): void;
  /**
   * Delete multiple rows at once
   * @param indices - JSON array of row indices to delete, e.g., "[0, 2, 5]"
   */
  delete_rows(indices_json: string): void;
  /**
   * Get unique row indices from selected cells
   * Returns JSON array of row indices, e.g., "[0, 2, 5]"
   */
  get_selected_row_indices(): string;
  /**
   * Insert a column at the specified position
   */
  insert_column(at_index: number): void;
  /**
   * Delete a column at the specified position
   */
  delete_column(index: number): void;
  /**
   * Delete all empty rows (rows with no non-empty cells)
   */
  delete_empty_rows(): number;
  /**
   * Check if a row is empty (all cells are empty)
   */
  is_row_empty(row: number): boolean;
  /**
   * Find all modified (edited) cells
   */
  find_modified_cells(): number;
  /**
   * Clear modified flags from all cells
   */
  clear_all_modified_flags(): void;
  /**
   * Check if a cell is modified
   */
  is_cell_modified(row: number, col: number): boolean;
  /**
   * Get count of modified cells
   */
  get_modified_cell_count(): number;
  /**
   * Search for text in grid cells (case-insensitive by default)
   */
  search_text(query: string): number;
  /**
   * Search for text with options
   */
  search_text_with_options(query: string, case_sensitive: boolean, whole_word: boolean): number;
  /**
   * Move to next search result
   */
  search_next(): boolean;
  /**
   * Move to previous search result
   */
  search_prev(): boolean;
  /**
   * Search using regular expression
   */
  search_regex(pattern: string, case_sensitive: boolean): number;
  /**
   * Validate regex pattern without performing search
   */
  validate_regex_pattern(pattern: string): boolean;
  /**
   * Clear search results
   */
  clear_search(): void;
  /**
   * Get search result count
   */
  get_search_result_count(): number;
  /**
   * Get current search index (1-based for display)
   */
  get_current_search_index(): number;
  /**
   * Check if a cell is a search result
   */
  is_search_result(row: number, col: number): boolean;
  /**
   * Check if a cell is the current (active) search result
   */
  is_current_search_result(row: number, col: number): boolean;
  /**
   * Replace current search result with new text
   */
  replace_current(replacement: string): boolean;
  /**
   * Replace all search results with new text
   */
  replace_all(replacement: string): number;
  /**
   * Replace in selection only
   */
  replace_in_selection(search: string, replacement: string, case_sensitive: boolean): number;
  /**
   * Sort by column
   */
  sort_by_column(col: number, ascending: boolean): void;
  /**
   * Toggle sort on column (click column header)
   */
  toggle_column_sort(col: number): void;
  /**
   * Get sort state for a column as JSON object
   * Returns: "{\"is_sorted\": true/false, \"is_ascending\": true/false}"
   */
  get_column_sort_state(col: number): string;
  /**
   * Add column to multi-column sort (for Shift+Click)
   */
  add_multi_column_sort(col: number, ascending: boolean): void;
  /**
   * Toggle column in multi-column sort
   */
  toggle_multi_column_sort(col: number): void;
  /**
   * Clear multi-column sort
   */
  clear_multi_column_sort(): void;
  /**
   * Get multi-column sort state as JSON array of [col, ascending] pairs
   * Returns: "[[col1, 1], [col2, 0], ...]" where 1=ascending, 0=descending
   */
  get_multi_column_sort_state(): string;
  /**
   * Check if a column is in multi-column sort
   * Returns JSON: "{\"is_sorted\": bool, \"is_ascending\": bool, \"sort_priority\": number}"
   */
  get_column_multi_sort_state(col: number): string;
  /**
   * Freeze first N rows
   */
  freeze_rows(count: number): void;
  /**
   * Freeze first N columns
   */
  freeze_cols(count: number): void;
  /**
   * Get frozen row count
   */
  get_frozen_rows(): number;
  /**
   * Set frozen row count
   */
  set_frozen_rows(count: number): void;
  /**
   * Get frozen column count
   */
  get_frozen_cols(): number;
  /**
   * Set frozen column count
   */
  set_frozen_cols(count: number): void;
  /**
   * Undo last edit action
   */
  undo(): boolean;
  /**
   * Redo last undone action
   */
  redo(): boolean;
  /**
   * Check if undo is available
   */
  can_undo(): boolean;
  /**
   * Check if redo is available
   */
  can_redo(): boolean;
  /**
   * Get undo stack size
   */
  get_undo_count(): number;
  /**
   * Get redo stack size
   */
  get_redo_count(): number;
  /**
   * Auto-fit column width to content
   */
  auto_fit_column(col: number): void;
  /**
   * Auto-fit all columns to content
   */
  auto_fit_all_columns(): void;
  /**
   * Set all columns to equal width
   */
  set_all_columns_equal_width(width: number): void;
  /**
   * Filter column by text (case-insensitive contains)
   */
  filter_column_by_text(col: number, text: string): void;
  /**
   * Filter column by empty cells
   */
  filter_column_show_non_empty(col: number): void;
  /**
   * Clear all column filters
   */
  clear_column_filters(): void;
  /**
   * Check if a row is filtered (hidden)
   */
  is_row_filtered(row: number): boolean;
  /**
   * Get visible row count
   */
  get_visible_row_count(): number;
  /**
   * Start performance benchmark (returns start time)
   */
  benchmark_start(): number;
  /**
   * End performance benchmark and return elapsed time in ms
   */
  benchmark_end(start_time: number): number;
  /**
   * Update FPS tracking (call this in render loop)
   */
  update_performance_metrics(): void;
  /**
   * Get current FPS
   */
  get_fps(): number;
  /**
   * Get total frame count
   */
  get_frame_count(): number;
  /**
   * Get last render time in ms
   */
  get_last_render_time(): number;
  /**
   * Reset performance metrics
   */
  reset_performance_metrics(): void;
  /**
   * Run performance benchmark (render N frames and return average time)
   */
  run_benchmark(frame_count: number): number;
  /**
   * Mark a specific cell as dirty (needs re-rendering)
   */
  mark_cell_dirty(row: number, col: number): void;
  /**
   * Mark all cells as dirty (force full re-render)
   */
  mark_all_dirty(): void;
  /**
   * Clear dirty cells (after rendering)
   */
  clear_dirty_cells(): void;
  /**
   * Get count of dirty cells
   */
  get_dirty_cell_count(): number;
  /**
   * Check if full render is needed
   */
  needs_full_render(): boolean;
  /**
   * Optimize memory by reserving capacity for expected data size
   */
  reserve_capacity(expected_cells: number): void;
  /**
   * Clear all non-essential cached data to free memory
   */
  clear_caches(): void;
  /**
   * Get estimated memory usage in bytes (approximate)
   */
  get_memory_usage(): number;
  /**
   * Compact memory by removing unused allocations
   */
  compact_memory(): void;
  /**
   * Export grid data as JSON for worker thread processing
   * Returns JSON array: [{"row":0,"col":0,"value":"text","type":"text"}, ...]
   */
  export_grid_data_json(): string;
  /**
   * Export a specific range of data as JSON for worker processing
   * Returns JSON array for the specified range
   */
  export_range_json(start_row: number, end_row: number, start_col: number, end_col: number): string;
  /**
   * Import processed data from worker thread
   * Accepts JSON array: [{"row":0,"col":0,"value":"text","type":"text"}, ...]
   */
  import_worker_result(result_json: string): number;
  /**
   * Get grid metadata for worker thread (dimensions, frozen areas, etc.)
   */
  get_grid_metadata_json(): string;
  /**
   * Prepare data for sorting in worker thread
   * Returns JSON with data and sort configuration
   * sort_columns_json: JSON array of column indices, e.g. "[0, 1]"
   * ascending_json: JSON array of booleans, e.g. "[true, false]"
   */
  prepare_sort_data(sort_columns_json: string, ascending_json: string): string;
  /**
   * Apply sorted row indices from worker result
   * Takes array of row indices representing the new order
   */
  apply_sorted_indices(indices_json: string): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_datagrid_free: (a: number, b: number) => void;
  readonly datagrid_from_container: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly datagrid_new: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number];
  readonly datagrid_render: (a: number) => void;
  readonly datagrid_resize: (a: number, b: number, c: number) => void;
  readonly datagrid_handle_wheel: (a: number, b: any) => void;
  readonly datagrid_handle_mouse_down_with_modifiers: (a: number, b: any, c: number, d: number) => void;
  readonly datagrid_handle_mouse_down: (a: number, b: any) => void;
  readonly datagrid_handle_mouse_down_at_with_modifiers: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly datagrid_handle_mouse_down_at: (a: number, b: number, c: number) => void;
  readonly datagrid_handle_mouse_up: (a: number, b: any) => void;
  readonly datagrid_handle_mouse_up_at: (a: number, b: number, c: number) => void;
  readonly datagrid_handle_mouse_move: (a: number, b: any) => void;
  readonly datagrid_handle_context_menu: (a: number, b: any) => [number, number];
  readonly datagrid_get_row_context_operations: (a: number, b: number) => [number, number];
  readonly datagrid_execute_row_operation: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly datagrid_set_cell_value: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly datagrid_get_cell_value: (a: number, b: number, c: number) => [number, number];
  readonly datagrid_get_dimensions: (a: number) => [number, number];
  readonly datagrid_get_viewport_info: (a: number) => [number, number];
  readonly datagrid_get_visible_range: (a: number) => [number, number];
  readonly datagrid_get_viewport_info_array: (a: number) => [number, number];
  readonly datagrid_get_max_scroll: (a: number) => [number, number];
  readonly datagrid_get_total_size: (a: number) => [number, number];
  readonly datagrid_set_scroll: (a: number, b: number, c: number) => void;
  readonly datagrid_set_cells_batch: (a: number, b: number, c: number) => [number, number];
  readonly datagrid_load_data_json: (a: number, b: number, c: number) => [number, number];
  readonly datagrid_is_range_loaded: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly datagrid_handle_keyboard: (a: number, b: any) => number;
  readonly datagrid_handle_keyboard_with_modifiers: (a: number, b: any, c: number) => number;
  readonly datagrid_handle_keyboard_with_modifiers_key: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly datagrid_start_edit: (a: number, b: number, c: number) => number;
  readonly datagrid_end_edit: (a: number) => void;
  readonly datagrid_is_editing: (a: number) => number;
  readonly datagrid_update_cell_value: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly datagrid_get_cell_edit_rect: (a: number, b: number, c: number) => [number, number];
  readonly datagrid_handle_double_click: (a: number, b: any) => [number, number];
  readonly datagrid_check_resize_handle: (a: number, b: number, c: number) => [number, number];
  readonly datagrid_start_resize: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly datagrid_update_resize: (a: number, b: number, c: number) => void;
  readonly datagrid_end_resize: (a: number) => void;
  readonly datagrid_is_resizing: (a: number) => number;
  readonly datagrid_get_selected_cells: (a: number) => [number, number];
  readonly datagrid_get_selection_count: (a: number) => number;
  readonly datagrid_select_all: (a: number) => void;
  readonly datagrid_select_row: (a: number, b: number) => void;
  readonly datagrid_select_col: (a: number, b: number) => void;
  readonly datagrid_copy_selected_cells: (a: number) => [number, number];
  readonly datagrid_cut_selected_cells: (a: number) => [number, number];
  readonly datagrid_paste_cells: (a: number, b: number, c: number) => [number, number];
  readonly datagrid_set_cell_bg_color: (a: number, b: number, c: number, d: number) => void;
  readonly datagrid_set_cell_fg_color: (a: number, b: number, c: number, d: number) => void;
  readonly datagrid_set_cell_font_style: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly datagrid_clear_cell_bg_color: (a: number, b: number, c: number) => void;
  readonly datagrid_clear_cell_fg_color: (a: number, b: number, c: number) => void;
  readonly datagrid_set_cell_style: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly datagrid_set_cell_border: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly datagrid_set_cell_borders: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly datagrid_clear_cell_border: (a: number, b: number, c: number, d: number) => void;
  readonly datagrid_add_column_group: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly datagrid_clear_column_groups: (a: number) => void;
  readonly datagrid_set_header_row_height: (a: number, b: number) => void;
  readonly datagrid_get_header_levels: (a: number) => number;
  readonly datagrid_get_header_height: (a: number) => number;
  readonly datagrid_set_column_validation: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly datagrid_clear_column_validation: (a: number, b: number) => void;
  readonly datagrid_get_column_validation: (a: number, b: number) => [number, number];
  readonly datagrid_set_column_editable: (a: number, b: number, c: number) => void;
  readonly datagrid_is_column_editable: (a: number, b: number) => number;
  readonly datagrid_get_all_column_editable_status: (a: number) => [number, number];
  readonly datagrid_set_column_name: (a: number, b: number, c: number, d: number) => void;
  readonly datagrid_insert_row: (a: number, b: number) => void;
  readonly datagrid_delete_row: (a: number, b: number) => void;
  readonly datagrid_delete_rows: (a: number, b: number, c: number) => [number, number];
  readonly datagrid_get_selected_row_indices: (a: number) => [number, number];
  readonly datagrid_insert_column: (a: number, b: number) => void;
  readonly datagrid_delete_column: (a: number, b: number) => void;
  readonly datagrid_delete_empty_rows: (a: number) => number;
  readonly datagrid_is_row_empty: (a: number, b: number) => number;
  readonly datagrid_find_modified_cells: (a: number) => number;
  readonly datagrid_clear_all_modified_flags: (a: number) => void;
  readonly datagrid_is_cell_modified: (a: number, b: number, c: number) => number;
  readonly datagrid_get_modified_cell_count: (a: number) => number;
  readonly datagrid_search_text: (a: number, b: number, c: number) => number;
  readonly datagrid_search_text_with_options: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly datagrid_search_next: (a: number) => number;
  readonly datagrid_search_prev: (a: number) => number;
  readonly datagrid_search_regex: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly datagrid_validate_regex_pattern: (a: number, b: number, c: number) => number;
  readonly datagrid_clear_search: (a: number) => void;
  readonly datagrid_get_search_result_count: (a: number) => number;
  readonly datagrid_get_current_search_index: (a: number) => number;
  readonly datagrid_is_search_result: (a: number, b: number, c: number) => number;
  readonly datagrid_is_current_search_result: (a: number, b: number, c: number) => number;
  readonly datagrid_replace_current: (a: number, b: number, c: number) => number;
  readonly datagrid_replace_all: (a: number, b: number, c: number) => number;
  readonly datagrid_replace_in_selection: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly datagrid_sort_by_column: (a: number, b: number, c: number) => void;
  readonly datagrid_toggle_column_sort: (a: number, b: number) => void;
  readonly datagrid_get_column_sort_state: (a: number, b: number) => [number, number];
  readonly datagrid_add_multi_column_sort: (a: number, b: number, c: number) => void;
  readonly datagrid_toggle_multi_column_sort: (a: number, b: number) => void;
  readonly datagrid_clear_multi_column_sort: (a: number) => void;
  readonly datagrid_get_multi_column_sort_state: (a: number) => [number, number];
  readonly datagrid_get_column_multi_sort_state: (a: number, b: number) => [number, number];
  readonly datagrid_freeze_rows: (a: number, b: number) => void;
  readonly datagrid_freeze_cols: (a: number, b: number) => void;
  readonly datagrid_get_frozen_rows: (a: number) => number;
  readonly datagrid_set_frozen_rows: (a: number, b: number) => void;
  readonly datagrid_get_frozen_cols: (a: number) => number;
  readonly datagrid_set_frozen_cols: (a: number, b: number) => void;
  readonly datagrid_undo: (a: number) => number;
  readonly datagrid_redo: (a: number) => number;
  readonly datagrid_can_undo: (a: number) => number;
  readonly datagrid_can_redo: (a: number) => number;
  readonly datagrid_get_undo_count: (a: number) => number;
  readonly datagrid_get_redo_count: (a: number) => number;
  readonly datagrid_auto_fit_column: (a: number, b: number) => void;
  readonly datagrid_auto_fit_all_columns: (a: number) => void;
  readonly datagrid_set_all_columns_equal_width: (a: number, b: number) => void;
  readonly datagrid_filter_column_by_text: (a: number, b: number, c: number, d: number) => void;
  readonly datagrid_filter_column_show_non_empty: (a: number, b: number) => void;
  readonly datagrid_clear_column_filters: (a: number) => void;
  readonly datagrid_is_row_filtered: (a: number, b: number) => number;
  readonly datagrid_get_visible_row_count: (a: number) => number;
  readonly datagrid_benchmark_start: (a: number) => number;
  readonly datagrid_benchmark_end: (a: number, b: number) => number;
  readonly datagrid_update_performance_metrics: (a: number) => void;
  readonly datagrid_get_fps: (a: number) => number;
  readonly datagrid_get_frame_count: (a: number) => number;
  readonly datagrid_get_last_render_time: (a: number) => number;
  readonly datagrid_reset_performance_metrics: (a: number) => void;
  readonly datagrid_run_benchmark: (a: number, b: number) => number;
  readonly datagrid_mark_cell_dirty: (a: number, b: number, c: number) => void;
  readonly datagrid_mark_all_dirty: (a: number) => void;
  readonly datagrid_clear_dirty_cells: (a: number) => void;
  readonly datagrid_get_dirty_cell_count: (a: number) => number;
  readonly datagrid_needs_full_render: (a: number) => number;
  readonly datagrid_reserve_capacity: (a: number, b: number) => void;
  readonly datagrid_clear_caches: (a: number) => void;
  readonly datagrid_get_memory_usage: (a: number) => number;
  readonly datagrid_compact_memory: (a: number) => void;
  readonly datagrid_export_grid_data_json: (a: number) => [number, number];
  readonly datagrid_export_range_json: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly datagrid_import_worker_result: (a: number, b: number, c: number) => [number, number, number];
  readonly datagrid_get_grid_metadata_json: (a: number) => [number, number];
  readonly datagrid_prepare_sort_data: (a: number, b: number, c: number, d: number, e: number) => [number, number, number, number];
  readonly datagrid_apply_sorted_indices: (a: number, b: number, c: number) => [number, number];
  readonly init: () => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;

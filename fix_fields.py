#!/usr/bin/env python3
"""Fix field access paths after refactoring"""

import re

# Read the file
with open('src/lib.rs', 'r') as f:
    content = f.read()

# Define replacements
replacements = [
    # Editing fields
    (r'\bself\.editing_row\b', 'self.editing.editing_row'),
    (r'\bself\.editing_col\b', 'self.editing.editing_col'),
    (r'\bself\.is_editing\b', 'self.editing.is_editing'),
    (r'\bself\.original_value\b', 'self.editing.original_value'),
    (r'\bself\.editing_cell\b', 'self.editing.editing_cell'),

    # Selection fields
    (r'\bself\.selected_cells\b', 'self.selection.selected_cells'),
    (r'\bself\.anchor_row\b', 'self.selection.anchor_row'),
    (r'\bself\.anchor_col\b', 'self.selection.anchor_col'),
    (r'\bself\.selection_anchor\b', 'self.selection.selection_anchor'),

    # Search fields
    (r'\bself\.search_query\b', 'self.search.search_query'),
    (r'\bself\.search_results\b', 'self.search.search_results'),
    (r'\bself\.current_search_index\b', 'self.search.current_search_index'),
    (r'\bself\.search_case_sensitive\b', 'self.search.search_case_sensitive'),
    (r'\bself\.search_whole_word\b', 'self.search.search_whole_word'),

    # Undo/Redo fields
    (r'\bself\.undo_stack\b', 'self.undo_redo.undo_stack'),
    (r'\bself\.redo_stack\b', 'self.undo_redo.redo_stack'),

    # Resize fields
    (r'\bself\.resize_col\b', 'self.resize.resize_col'),
    (r'\bself\.resize_start_x\b', 'self.resize.resize_start_x'),
    (r'\bself\.resize_start_width\b', 'self.resize.resize_start_width'),
    (r'\bself\.is_resizing\b', 'self.resize.is_resizing'),
    (r'\bself\.resizing_column\b', 'self.resize.resizing_column'),
    (r'\bself\.resizing_row\b', 'self.resize.resizing_row'),
    (r'\bself\.resize_start_pos\b', 'self.resize.resize_start_pos'),
    (r'\bself\.resize_start_size\b', 'self.resize.resize_start_size'),
]

# Apply replacements
for pattern, replacement in replacements:
    content = re.sub(pattern, replacement, content)

# Write back
with open('src/lib.rs', 'w') as f:
    f.write(content)

print("Fixed field access paths in lib.rs")

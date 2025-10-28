/// Mouse event handler for grid interaction
pub struct MouseHandler {
    pub is_dragging: bool,
    pub last_x: f32,
    pub last_y: f32,
    pub selected_cell: Option<(usize, usize)>,
}

impl MouseHandler {
    pub fn new() -> Self {
        Self {
            is_dragging: false,
            last_x: 0.0,
            last_y: 0.0,
            selected_cell: None,
        }
    }

    pub fn mouse_down(&mut self, x: f32, y: f32) {
        self.is_dragging = true;
        self.last_x = x;
        self.last_y = y;
    }

    pub fn mouse_up(&mut self) {
        self.is_dragging = false;
    }

    pub fn mouse_move(&mut self, x: f32, y: f32) -> Option<(f32, f32)> {
        if self.is_dragging {
            let dx = x - self.last_x;
            let dy = y - self.last_y;
            self.last_x = x;
            self.last_y = y;
            Some((dx, dy))
        } else {
            None
        }
    }

    pub fn select_cell(&mut self, row: usize, col: usize) {
        self.selected_cell = Some((row, col));
    }
}

impl Default for MouseHandler {
    fn default() -> Self {
        Self::new()
    }
}
